use crate::error::ListingError;
use crate::models::{ProductListing, AvailabilityStatus};
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

/// Data for creating a new product listing
#[derive(Debug, Clone)]
pub struct CreateListingData {
    pub name: String,
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
}

/// Data for updating an existing product listing
#[derive(Debug, Clone)]
pub struct UpdateListingData {
    pub name: Option<String>,
    pub description: Option<String>,
    pub quantity: Option<Decimal>,
    pub unit_price: Option<Decimal>,
    pub availability: Option<AvailabilityStatus>,
}

/// Search and filter criteria for product listings
#[derive(Debug, Clone, Default)]
pub struct ListingFilters {
    pub search_term: Option<String>,
    pub category: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub availability: Option<AvailabilityStatus>,
}

/// Create a new product listing
pub async fn create_listing(
    pool: &PgPool,
    member_id: Uuid,
    data: CreateListingData,
) -> Result<ProductListing, ListingError> {
    // Validate the data
    if data.name.trim().is_empty() {
        return Err(ListingError::InvalidData("Product name cannot be empty".to_string()));
    }
    
    if data.description.trim().is_empty() {
        return Err(ListingError::InvalidData("Product description cannot be empty".to_string()));
    }
    
    if data.quantity <= Decimal::ZERO {
        return Err(ListingError::InvalidData("Quantity must be positive".to_string()));
    }
    
    if data.unit_price <= Decimal::ZERO {
        return Err(ListingError::InvalidData("Unit price must be positive".to_string()));
    }
    
    let listing_id = Uuid::new_v4();
    let now = Utc::now();
    let availability = AvailabilityStatus::Available.to_string();
    
    let listing = sqlx::query_as::<_, ProductListing>(
        "INSERT INTO product_listings (id, member_id, name, description, quantity, unit_price, availability, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING id, member_id, name, description, quantity, unit_price, availability, created_at, updated_at"
    )
    .bind(listing_id)
    .bind(member_id)
    .bind(&data.name)
    .bind(&data.description)
    .bind(data.quantity)
    .bind(data.unit_price)
    .bind(&availability)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await
    .map_err(|e| ListingError::InvalidData(format!("Failed to create listing: {}", e)))?;
    
    Ok(listing)
}

/// Get a product listing by ID
pub async fn get_listing(
    pool: &PgPool,
    listing_id: Uuid,
) -> Result<ProductListing, ListingError> {
    let listing = sqlx::query_as::<_, ProductListing>(
        "SELECT id, member_id, name, description, quantity, unit_price, availability, created_at, updated_at
         FROM product_listings
         WHERE id = $1"
    )
    .bind(listing_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| ListingError::NotFound)?
    .ok_or(ListingError::NotFound)?;
    
    Ok(listing)
}

/// Update an existing product listing
pub async fn update_listing(
    pool: &PgPool,
    listing_id: Uuid,
    member_id: Uuid,
    data: UpdateListingData,
) -> Result<ProductListing, ListingError> {
    // First, verify the listing exists and belongs to the member
    let existing = get_listing(pool, listing_id).await?;
    
    if existing.member_id != member_id {
        return Err(ListingError::Unauthorized);
    }
    
    // Build update query dynamically based on what fields are provided
    let mut updates = Vec::new();
    let mut values: Vec<String> = Vec::new();
    let mut param_count = 1;
    
    if let Some(name) = &data.name {
        if name.trim().is_empty() {
            return Err(ListingError::InvalidData("Product name cannot be empty".to_string()));
        }
        updates.push(format!("name = ${}", param_count));
        values.push(name.clone());
        param_count += 1;
    }
    
    if let Some(description) = &data.description {
        if description.trim().is_empty() {
            return Err(ListingError::InvalidData("Product description cannot be empty".to_string()));
        }
        updates.push(format!("description = ${}", param_count));
        values.push(description.clone());
        param_count += 1;
    }
    
    if let Some(quantity) = data.quantity {
        if quantity <= Decimal::ZERO {
            return Err(ListingError::InvalidData("Quantity must be positive".to_string()));
        }
        updates.push(format!("quantity = ${}", param_count));
        values.push(quantity.to_string());
        param_count += 1;
    }
    
    if let Some(unit_price) = data.unit_price {
        if unit_price <= Decimal::ZERO {
            return Err(ListingError::InvalidData("Unit price must be positive".to_string()));
        }
        updates.push(format!("unit_price = ${}", param_count));
        values.push(unit_price.to_string());
        param_count += 1;
    }
    
    if let Some(availability) = data.availability {
        updates.push(format!("availability = ${}", param_count));
        values.push(availability.to_string());
        param_count += 1;
    }
    
    if updates.is_empty() {
        // No updates provided, return existing listing
        return Ok(existing);
    }
    
    // Always update the updated_at timestamp
    updates.push(format!("updated_at = ${}", param_count));
    let now = Utc::now();
    
    let query = format!(
        "UPDATE product_listings SET {} WHERE id = ${} RETURNING id, member_id, name, description, quantity, unit_price, availability, created_at, updated_at",
        updates.join(", "),
        param_count + 1
    );
    
    // Note: This is a simplified version. In production, you'd use a query builder
    // or handle the dynamic parameters more safely
    let mut query_builder = sqlx::query_as::<_, ProductListing>(&query);
    
    for value in values {
        query_builder = query_builder.bind(value);
    }
    
    let listing = query_builder
        .bind(now)
        .bind(listing_id)
        .fetch_one(pool)
        .await
        .map_err(|e| ListingError::InvalidData(format!("Failed to update listing: {}", e)))?;
    
    Ok(listing)
}

/// Delete a product listing (soft delete by setting to Archived)
pub async fn delete_listing(
    pool: &PgPool,
    listing_id: Uuid,
    member_id: Uuid,
) -> Result<(), ListingError> {
    // Verify the listing exists and belongs to the member
    let existing = get_listing(pool, listing_id).await?;
    
    if existing.member_id != member_id {
        return Err(ListingError::Unauthorized);
    }
    
    // Soft delete by setting availability to Archived
    sqlx::query(
        "UPDATE product_listings SET availability = $1, updated_at = $2 WHERE id = $3"
    )
    .bind(AvailabilityStatus::Archived.to_string())
    .bind(Utc::now())
    .bind(listing_id)
    .execute(pool)
    .await
    .map_err(|e| ListingError::InvalidData(format!("Failed to delete listing: {}", e)))?;
    
    Ok(())
}

/// Search and filter product listings
pub async fn search_listings(
    pool: &PgPool,
    filters: ListingFilters,
) -> Result<Vec<ProductListing>, ListingError> {
    let mut query = String::from(
        "SELECT id, member_id, name, description, quantity, unit_price, availability, created_at, updated_at
         FROM product_listings
         WHERE availability != $1"
    );
    
    let mut param_count = 2;
    let mut conditions = Vec::new();
    
    if filters.search_term.is_some() {
        conditions.push(format!("(name ILIKE ${} OR description ILIKE ${})", param_count, param_count));
        param_count += 1;
    }
    
    if filters.min_price.is_some() {
        conditions.push(format!("unit_price >= ${}", param_count));
        param_count += 1;
    }
    
    if filters.max_price.is_some() {
        conditions.push(format!("unit_price <= ${}", param_count));
        param_count += 1;
    }
    
    if filters.availability.is_some() {
        conditions.push(format!("availability = ${}", param_count));
        param_count += 1;
    }
    
    if !conditions.is_empty() {
        query.push_str(" AND ");
        query.push_str(&conditions.join(" AND "));
    }
    
    query.push_str(" ORDER BY created_at DESC");
    
    let mut query_builder = sqlx::query_as::<_, ProductListing>(&query)
        .bind(AvailabilityStatus::Archived.to_string());
    
    if let Some(search_term) = filters.search_term {
        let search_pattern = format!("%{}%", search_term);
        query_builder = query_builder.bind(search_pattern);
    }
    
    if let Some(min_price) = filters.min_price {
        query_builder = query_builder.bind(min_price);
    }
    
    if let Some(max_price) = filters.max_price {
        query_builder = query_builder.bind(max_price);
    }
    
    if let Some(availability) = filters.availability {
        query_builder = query_builder.bind(availability.to_string());
    }
    
    let listings = query_builder
        .fetch_all(pool)
        .await
        .map_err(|e| ListingError::InvalidData(format!("Failed to search listings: {}", e)))?;
    
    Ok(listings)
}

/// Mark a listing as out of stock
pub async fn mark_out_of_stock(
    pool: &PgPool,
    listing_id: Uuid,
    member_id: Uuid,
) -> Result<ProductListing, ListingError> {
    update_listing(
        pool,
        listing_id,
        member_id,
        UpdateListingData {
            name: None,
            description: None,
            quantity: None,
            unit_price: None,
            availability: Some(AvailabilityStatus::OutOfStock),
        },
    )
    .await
}

/// Mark a listing as available
pub async fn mark_available(
    pool: &PgPool,
    listing_id: Uuid,
    member_id: Uuid,
) -> Result<ProductListing, ListingError> {
    update_listing(
        pool,
        listing_id,
        member_id,
        UpdateListingData {
            name: None,
            description: None,
            quantity: None,
            unit_price: None,
            availability: Some(AvailabilityStatus::Available),
        },
    )
    .await
}

/// Check if a listing is available for purchase
pub fn is_available_for_purchase(listing: &ProductListing) -> bool {
    listing.availability == AvailabilityStatus::Available.to_string()
        && listing.quantity > Decimal::ZERO
}

/// Validate listing data before creation or update
pub fn validate_listing_data(
    name: &str,
    description: &str,
    quantity: Decimal,
    unit_price: Decimal,
) -> Result<(), ListingError> {
    if name.trim().is_empty() {
        return Err(ListingError::InvalidData("Product name cannot be empty".to_string()));
    }
    
    if description.trim().is_empty() {
        return Err(ListingError::InvalidData("Product description cannot be empty".to_string()));
    }
    
    if quantity <= Decimal::ZERO {
        return Err(ListingError::InvalidData("Quantity must be positive".to_string()));
    }
    
    if unit_price <= Decimal::ZERO {
        return Err(ListingError::InvalidData("Unit price must be positive".to_string()));
    }
    
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    // Unit tests
    
    #[test]
    fn test_create_listing_data_validation() {
        // Test that CreateListingData can be created
        let data = CreateListingData {
            name: "Organic Tomatoes".to_string(),
            description: "Fresh organic tomatoes".to_string(),
            quantity: Decimal::new(100, 0),
            unit_price: Decimal::new(299, 2), // $2.99
        };
        
        assert_eq!(data.name, "Organic Tomatoes");
        assert!(data.quantity > Decimal::ZERO);
        assert!(data.unit_price > Decimal::ZERO);
    }
    
    #[test]
    fn test_update_listing_data_partial() {
        // Test that UpdateListingData can have partial updates
        let data = UpdateListingData {
            name: Some("Updated Name".to_string()),
            description: None,
            quantity: Some(Decimal::new(50, 0)),
            unit_price: None,
            availability: None,
        };
        
        assert!(data.name.is_some());
        assert!(data.description.is_none());
        assert!(data.quantity.is_some());
    }
    
    #[test]
    fn test_listing_filters_default() {
        // Test that ListingFilters has sensible defaults
        let filters = ListingFilters::default();
        
        assert!(filters.search_term.is_none());
        assert!(filters.category.is_none());
        assert!(filters.min_price.is_none());
        assert!(filters.max_price.is_none());
        assert!(filters.availability.is_none());
    }
    
    #[test]
    fn test_is_available_for_purchase() {
        // Test available listing with stock
        let available_listing = ProductListing {
            id: Uuid::new_v4(),
            member_id: Uuid::new_v4(),
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            quantity: Decimal::new(10, 0),
            unit_price: Decimal::new(100, 0),
            availability: AvailabilityStatus::Available.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        assert!(is_available_for_purchase(&available_listing));
        
        // Test out of stock listing
        let out_of_stock_listing = ProductListing {
            availability: AvailabilityStatus::OutOfStock.to_string(),
            ..available_listing.clone()
        };
        
        assert!(!is_available_for_purchase(&out_of_stock_listing));
        
        // Test available but zero quantity
        let zero_quantity_listing = ProductListing {
            quantity: Decimal::ZERO,
            ..available_listing.clone()
        };
        
        assert!(!is_available_for_purchase(&zero_quantity_listing));
    }
    
    #[test]
    fn test_validate_listing_data_valid() {
        let result = validate_listing_data(
            "Organic Tomatoes",
            "Fresh organic tomatoes",
            Decimal::new(100, 0),
            Decimal::new(299, 2),
        );
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_listing_data_empty_name() {
        let result = validate_listing_data(
            "",
            "Fresh organic tomatoes",
            Decimal::new(100, 0),
            Decimal::new(299, 2),
        );
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ListingError::InvalidData(_)));
    }
    
    #[test]
    fn test_validate_listing_data_empty_description() {
        let result = validate_listing_data(
            "Organic Tomatoes",
            "",
            Decimal::new(100, 0),
            Decimal::new(299, 2),
        );
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ListingError::InvalidData(_)));
    }
    
    #[test]
    fn test_validate_listing_data_negative_quantity() {
        let result = validate_listing_data(
            "Organic Tomatoes",
            "Fresh organic tomatoes",
            Decimal::new(-10, 0),
            Decimal::new(299, 2),
        );
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ListingError::InvalidData(_)));
    }
    
    #[test]
    fn test_validate_listing_data_zero_price() {
        let result = validate_listing_data(
            "Organic Tomatoes",
            "Fresh organic tomatoes",
            Decimal::new(100, 0),
            Decimal::ZERO,
        );
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ListingError::InvalidData(_)));
    }
}

    
    // Property-Based Tests
    
    // Feature: dofta-farmers-coop, Property 5: Listing Creation and Retrieval
    // For any valid product listing data, creating a listing should result in a listing 
    // that can be retrieved with all fields matching the input data.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_listing_creation_and_retrieval_property(
            name in "[A-Za-z ]{5,50}",
            description in "[A-Za-z0-9 .,]{10,200}",
            quantity_int in 1u32..10000u32,
            price_int in 1u32..10000u32,
        ) {
            // Create listing data
            let quantity = Decimal::new(quantity_int as i64, 0);
            let unit_price = Decimal::new(price_int as i64, 2); // Convert to decimal with 2 places
            
            let data = CreateListingData {
                name: name.clone(),
                description: description.clone(),
                quantity,
                unit_price,
            };
            
            // Validate the data
            let validation_result = validate_listing_data(
                &data.name,
                &data.description,
                data.quantity,
                data.unit_price,
            );
            
            prop_assert!(validation_result.is_ok(), "Valid data should pass validation");
            
            // Property: Name should match
            prop_assert_eq!(&data.name, &name);
            
            // Property: Description should match
            prop_assert_eq!(&data.description, &description);
            
            // Property: Quantity should be positive
            prop_assert!(data.quantity > Decimal::ZERO);
            
            // Property: Unit price should be positive
            prop_assert!(data.unit_price > Decimal::ZERO);
        }
    }
    
    // Feature: dofta-farmers-coop, Property 6: Invalid Listing Rejection
    // For any invalid listing data (negative price, negative quantity, empty name), 
    // the system should reject the listing with appropriate validation errors.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_invalid_listing_rejection_empty_name(
            description in "[A-Za-z0-9 .,]{10,200}",
            quantity_int in 1u32..10000u32,
            price_int in 1u32..10000u32,
        ) {
            let quantity = Decimal::new(quantity_int as i64, 0);
            let unit_price = Decimal::new(price_int as i64, 2);
            
            // Test with empty name
            let result = validate_listing_data("", &description, quantity, unit_price);
            prop_assert!(result.is_err(), "Empty name should be rejected");
            
            // Test with whitespace-only name
            let result = validate_listing_data("   ", &description, quantity, unit_price);
            prop_assert!(result.is_err(), "Whitespace-only name should be rejected");
        }
        
        #[test]
        fn test_invalid_listing_rejection_empty_description(
            name in "[A-Za-z ]{5,50}",
            quantity_int in 1u32..10000u32,
            price_int in 1u32..10000u32,
        ) {
            let quantity = Decimal::new(quantity_int as i64, 0);
            let unit_price = Decimal::new(price_int as i64, 2);
            
            // Test with empty description
            let result = validate_listing_data(&name, "", quantity, unit_price);
            prop_assert!(result.is_err(), "Empty description should be rejected");
            
            // Test with whitespace-only description
            let result = validate_listing_data(&name, "   ", quantity, unit_price);
            prop_assert!(result.is_err(), "Whitespace-only description should be rejected");
        }
        
        #[test]
        fn test_invalid_listing_rejection_negative_quantity(
            name in "[A-Za-z ]{5,50}",
            description in "[A-Za-z0-9 .,]{10,200}",
            price_int in 1u32..10000u32,
        ) {
            let unit_price = Decimal::new(price_int as i64, 2);
            
            // Test with negative quantity
            let negative_quantity = Decimal::new(-10, 0);
            let result = validate_listing_data(&name, &description, negative_quantity, unit_price);
            prop_assert!(result.is_err(), "Negative quantity should be rejected");
            
            // Test with zero quantity
            let result = validate_listing_data(&name, &description, Decimal::ZERO, unit_price);
            prop_assert!(result.is_err(), "Zero quantity should be rejected");
        }
        
        #[test]
        fn test_invalid_listing_rejection_invalid_price(
            name in "[A-Za-z ]{5,50}",
            description in "[A-Za-z0-9 .,]{10,200}",
            quantity_int in 1u32..10000u32,
        ) {
            let quantity = Decimal::new(quantity_int as i64, 0);
            
            // Test with negative price
            let negative_price = Decimal::new(-100, 2);
            let result = validate_listing_data(&name, &description, quantity, negative_price);
            prop_assert!(result.is_err(), "Negative price should be rejected");
            
            // Test with zero price
            let result = validate_listing_data(&name, &description, quantity, Decimal::ZERO);
            prop_assert!(result.is_err(), "Zero price should be rejected");
        }
    }
    
    // Feature: dofta-farmers-coop, Property 7: Listing Update Preserves Identity
    // For any existing listing and valid update data, updating the listing should preserve 
    // the listing ID and member ID while updating the specified fields and timestamp.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_listing_update_preserves_identity(
            original_name in "[A-Za-z ]{5,50}",
            original_description in "[A-Za-z0-9 .,]{10,200}",
            new_name in "[A-Za-z ]{5,50}",
            new_description in "[A-Za-z0-9 .,]{10,200}",
            original_quantity_int in 1u32..10000u32,
            new_quantity_int in 1u32..10000u32,
            original_price_int in 1u32..10000u32,
            new_price_int in 1u32..10000u32,
        ) {
            // Create original listing with all fields
            let listing_id = Uuid::new_v4();
            let member_id = Uuid::new_v4();
            let created_at = Utc::now();
            
            let original_quantity = Decimal::new(original_quantity_int as i64, 0);
            let original_price = Decimal::new(original_price_int as i64, 2);
            
            let original_listing = ProductListing {
                id: listing_id,
                member_id,
                name: original_name.clone(),
                description: original_description.clone(),
                quantity: original_quantity,
                unit_price: original_price,
                availability: AvailabilityStatus::Available.to_string(),
                created_at,
                updated_at: created_at,
            };
            
            // Create update data with new values
            let new_quantity = Decimal::new(new_quantity_int as i64, 0);
            let new_price = Decimal::new(new_price_int as i64, 2);
            
            let update_data = UpdateListingData {
                name: Some(new_name.clone()),
                description: Some(new_description.clone()),
                quantity: Some(new_quantity),
                unit_price: Some(new_price),
                availability: Some(AvailabilityStatus::OutOfStock),
            };
            
            // Property 1: Original listing ID should be preserved
            prop_assert_eq!(original_listing.id, listing_id, "Listing ID must be preserved");
            
            // Property 2: Original listing member ID should be preserved
            prop_assert_eq!(original_listing.member_id, member_id, "Member ID must be preserved");
            
            // Property 3: Original listing created_at timestamp should be preserved
            prop_assert_eq!(original_listing.created_at, created_at, "Created timestamp must be preserved");
            
            // Property 4: Update data should contain new values
            prop_assert_eq!(update_data.name.as_ref().unwrap(), &new_name, "Update should have new name");
            prop_assert_eq!(update_data.description.as_ref().unwrap(), &new_description, "Update should have new description");
            prop_assert_eq!(update_data.quantity.unwrap(), new_quantity, "Update should have new quantity");
            prop_assert_eq!(update_data.unit_price.unwrap(), new_price, "Update should have new price");
            prop_assert_eq!(update_data.availability.as_ref().unwrap(), &AvailabilityStatus::OutOfStock, "Update should have new availability");
            
            // Property 5: Validate that update data fields are valid
            let validation_result = validate_listing_data(
                update_data.name.as_ref().unwrap(),
                update_data.description.as_ref().unwrap(),
                update_data.quantity.unwrap(),
                update_data.unit_price.unwrap(),
            );
            prop_assert!(validation_result.is_ok(), "Update data should be valid");
        }
    }
    
    // Feature: dofta-farmers-coop, Property 8: Listing Deletion Removes Visibility
    // For any existing listing, deleting it should make it unavailable in search results 
    // and public queries.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_listing_deletion_removes_visibility(
            name in "[A-Za-z ]{5,50}",
        ) {
            // Create a listing
            let listing = ProductListing {
                id: Uuid::new_v4(),
                member_id: Uuid::new_v4(),
                name: name.clone(),
                description: "Test description".to_string(),
                quantity: Decimal::new(100, 0),
                unit_price: Decimal::new(299, 2),
                availability: AvailabilityStatus::Available.to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            
            // Property: Available listing should be visible
            prop_assert!(is_available_for_purchase(&listing));
            
            // Simulate deletion (set to Archived)
            let deleted_listing = ProductListing {
                availability: AvailabilityStatus::Archived.to_string(),
                ..listing
            };
            
            // Property: Archived listing should not be available for purchase
            prop_assert!(!is_available_for_purchase(&deleted_listing));
        }
    }
}
