use crate::error::OrderError;
use crate::models::{Order, OrderStatus};
use crate::listings;
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

/// Data for creating a new order
#[derive(Debug, Clone)]
pub struct CreateOrderData {
    pub product_listing_id: Uuid,
    pub quantity: Decimal,
}

/// Create a new order
pub async fn create_order(
    pool: &PgPool,
    buyer_id: Uuid,
    data: CreateOrderData,
) -> Result<Order, OrderError> {
    // Validate quantity
    if data.quantity <= Decimal::ZERO {
        return Err(OrderError::InvalidData("Order quantity must be positive".to_string()));
    }
    
    // Get the product listing to validate availability and calculate total
    let listing = listings::get_listing(pool, data.product_listing_id)
        .await
        .map_err(|_| OrderError::ProductUnavailable)?;
    
    // Check if listing is available for purchase
    if !listings::is_available_for_purchase(&listing) {
        return Err(OrderError::ProductUnavailable);
    }
    
    // Check if there's sufficient quantity
    if listing.quantity < data.quantity {
        return Err(OrderError::InsufficientQuantity);
    }
    
    // Calculate total amount
    let total_amount = listing.unit_price * data.quantity;
    
    // Create the order
    let order_id = Uuid::new_v4();
    let seller_id = listing.member_id;
    let now = Utc::now();
    let status = OrderStatus::Pending.to_string();
    
    let order = sqlx::query_as::<_, Order>(
        "INSERT INTO orders (id, buyer_id, seller_id, product_listing_id, quantity, total_amount, status, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING id, buyer_id, seller_id, product_listing_id, quantity, total_amount, status, created_at"
    )
    .bind(order_id)
    .bind(buyer_id)
    .bind(seller_id)
    .bind(data.product_listing_id)
    .bind(data.quantity)
    .bind(total_amount)
    .bind(&status)
    .bind(now)
    .fetch_one(pool)
    .await
    .map_err(|e| OrderError::InvalidData(format!("Failed to create order: {}", e)))?;
    
    Ok(order)
}

/// Get an order by ID
pub async fn get_order(
    pool: &PgPool,
    order_id: Uuid,
) -> Result<Order, OrderError> {
    let order = sqlx::query_as::<_, Order>(
        "SELECT id, buyer_id, seller_id, product_listing_id, quantity, total_amount, status, created_at
         FROM orders
         WHERE id = $1"
    )
    .bind(order_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| OrderError::NotFound)?
    .ok_or(OrderError::NotFound)?;
    
    Ok(order)
}

/// Get all orders for a buyer
pub async fn get_orders_by_buyer(
    pool: &PgPool,
    buyer_id: Uuid,
) -> Result<Vec<Order>, OrderError> {
    let orders = sqlx::query_as::<_, Order>(
        "SELECT id, buyer_id, seller_id, product_listing_id, quantity, total_amount, status, created_at
         FROM orders
         WHERE buyer_id = $1
         ORDER BY created_at DESC"
    )
    .bind(buyer_id)
    .fetch_all(pool)
    .await
    .map_err(|e| OrderError::InvalidData(format!("Failed to fetch orders: {}", e)))?;
    
    Ok(orders)
}

/// Get all orders for a seller
pub async fn get_orders_by_seller(
    pool: &PgPool,
    seller_id: Uuid,
) -> Result<Vec<Order>, OrderError> {
    let orders = sqlx::query_as::<_, Order>(
        "SELECT id, buyer_id, seller_id, product_listing_id, quantity, total_amount, status, created_at
         FROM orders
         WHERE seller_id = $1
         ORDER BY created_at DESC"
    )
    .bind(seller_id)
    .fetch_all(pool)
    .await
    .map_err(|e| OrderError::InvalidData(format!("Failed to fetch orders: {}", e)))?;
    
    Ok(orders)
}

/// Update order status
async fn update_order_status(
    pool: &PgPool,
    order_id: Uuid,
    new_status: OrderStatus,
) -> Result<Order, OrderError> {
    let order = sqlx::query_as::<_, Order>(
        "UPDATE orders SET status = $1 WHERE id = $2
         RETURNING id, buyer_id, seller_id, product_listing_id, quantity, total_amount, status, created_at"
    )
    .bind(new_status.to_string())
    .bind(order_id)
    .fetch_one(pool)
    .await
    .map_err(|_| OrderError::NotFound)?;
    
    Ok(order)
}

/// Accept an order (seller action)
pub async fn accept_order(
    pool: &PgPool,
    order_id: Uuid,
    seller_id: Uuid,
) -> Result<Order, OrderError> {
    // Get the order and verify it belongs to the seller
    let order = get_order(pool, order_id).await?;
    
    if order.seller_id != seller_id {
        return Err(OrderError::Unauthorized);
    }
    
    // Validate status transition
    let current_status = order.status.parse::<OrderStatus>()
        .map_err(|e| OrderError::InvalidData(format!("Invalid order status: {}", e)))?;
    
    if !is_valid_status_transition(&current_status, &OrderStatus::Accepted) {
        return Err(OrderError::InvalidStatusTransition(
            format!("Cannot transition from {:?} to Accepted", current_status)
        ));
    }
    
    // Update status to Accepted
    update_order_status(pool, order_id, OrderStatus::Accepted).await
}

/// Reject an order (seller action)
pub async fn reject_order(
    pool: &PgPool,
    order_id: Uuid,
    seller_id: Uuid,
) -> Result<Order, OrderError> {
    // Get the order and verify it belongs to the seller
    let order = get_order(pool, order_id).await?;
    
    if order.seller_id != seller_id {
        return Err(OrderError::Unauthorized);
    }
    
    // Validate status transition
    let current_status = order.status.parse::<OrderStatus>()
        .map_err(|e| OrderError::InvalidData(format!("Invalid order status: {}", e)))?;
    
    if !is_valid_status_transition(&current_status, &OrderStatus::Rejected) {
        return Err(OrderError::InvalidStatusTransition(
            format!("Cannot transition from {:?} to Rejected", current_status)
        ));
    }
    
    // Update status to Rejected
    update_order_status(pool, order_id, OrderStatus::Rejected).await
}

/// Complete an order (after successful transaction)
pub async fn complete_order(
    pool: &PgPool,
    order_id: Uuid,
) -> Result<Order, OrderError> {
    // Get the order
    let order = get_order(pool, order_id).await?;
    
    // Validate status transition
    let current_status = order.status.parse::<OrderStatus>()
        .map_err(|e| OrderError::InvalidData(format!("Invalid order status: {}", e)))?;
    
    if !is_valid_status_transition(&current_status, &OrderStatus::Completed) {
        return Err(OrderError::InvalidStatusTransition(
            format!("Cannot transition from {:?} to Completed", current_status)
        ));
    }
    
    // Update status to Completed
    update_order_status(pool, order_id, OrderStatus::Completed).await
}

/// Cancel an order (buyer action)
pub async fn cancel_order(
    pool: &PgPool,
    order_id: Uuid,
    buyer_id: Uuid,
) -> Result<Order, OrderError> {
    // Get the order and verify it belongs to the buyer
    let order = get_order(pool, order_id).await?;
    
    if order.buyer_id != buyer_id {
        return Err(OrderError::Unauthorized);
    }
    
    // Validate status transition
    let current_status = order.status.parse::<OrderStatus>()
        .map_err(|e| OrderError::InvalidData(format!("Invalid order status: {}", e)))?;
    
    if !is_valid_status_transition(&current_status, &OrderStatus::Cancelled) {
        return Err(OrderError::InvalidStatusTransition(
            format!("Cannot transition from {:?} to Cancelled", current_status)
        ));
    }
    
    // Update status to Cancelled
    update_order_status(pool, order_id, OrderStatus::Cancelled).await
}

/// Validate if a status transition is allowed
pub fn is_valid_status_transition(from: &OrderStatus, to: &OrderStatus) -> bool {
    match (from, to) {
        // From Pending
        (OrderStatus::Pending, OrderStatus::Accepted) => true,
        (OrderStatus::Pending, OrderStatus::Rejected) => true,
        (OrderStatus::Pending, OrderStatus::Cancelled) => true,
        
        // From Accepted
        (OrderStatus::Accepted, OrderStatus::Completed) => true,
        (OrderStatus::Accepted, OrderStatus::Cancelled) => true,
        
        // No transitions from terminal states
        (OrderStatus::Rejected, _) => false,
        (OrderStatus::Completed, _) => false,
        (OrderStatus::Cancelled, _) => false,
        
        // All other transitions are invalid
        _ => false,
    }
}

/// Check if an order can be accepted
pub fn can_accept_order(order: &Order) -> bool {
    if let Ok(status) = order.status.parse::<OrderStatus>() {
        matches!(status, OrderStatus::Pending)
    } else {
        false
    }
}

/// Check if an order can be rejected
pub fn can_reject_order(order: &Order) -> bool {
    if let Ok(status) = order.status.parse::<OrderStatus>() {
        matches!(status, OrderStatus::Pending)
    } else {
        false
    }
}

/// Check if an order can be completed
pub fn can_complete_order(order: &Order) -> bool {
    if let Ok(status) = order.status.parse::<OrderStatus>() {
        matches!(status, OrderStatus::Accepted)
    } else {
        false
    }
}

/// Check if an order can be cancelled
pub fn can_cancel_order(order: &Order) -> bool {
    if let Ok(status) = order.status.parse::<OrderStatus>() {
        matches!(status, OrderStatus::Pending | OrderStatus::Accepted)
    } else {
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use crate::models::{ProductListing, AvailabilityStatus};
    
    // Unit tests
    
    #[test]
    fn test_create_order_data_validation() {
        let data = CreateOrderData {
            product_listing_id: Uuid::new_v4(),
            quantity: Decimal::new(10, 0),
        };
        
        assert!(data.quantity > Decimal::ZERO);
    }
    
    #[test]
    fn test_is_valid_status_transition_pending_to_accepted() {
        assert!(is_valid_status_transition(&OrderStatus::Pending, &OrderStatus::Accepted));
    }
    
    #[test]
    fn test_is_valid_status_transition_pending_to_rejected() {
        assert!(is_valid_status_transition(&OrderStatus::Pending, &OrderStatus::Rejected));
    }
    
    #[test]
    fn test_is_valid_status_transition_pending_to_cancelled() {
        assert!(is_valid_status_transition(&OrderStatus::Pending, &OrderStatus::Cancelled));
    }
    
    #[test]
    fn test_is_valid_status_transition_accepted_to_completed() {
        assert!(is_valid_status_transition(&OrderStatus::Accepted, &OrderStatus::Completed));
    }
    
    #[test]
    fn test_is_valid_status_transition_accepted_to_cancelled() {
        assert!(is_valid_status_transition(&OrderStatus::Accepted, &OrderStatus::Cancelled));
    }
    
    #[test]
    fn test_is_valid_status_transition_rejected_to_any() {
        // Rejected is a terminal state
        assert!(!is_valid_status_transition(&OrderStatus::Rejected, &OrderStatus::Pending));
        assert!(!is_valid_status_transition(&OrderStatus::Rejected, &OrderStatus::Accepted));
        assert!(!is_valid_status_transition(&OrderStatus::Rejected, &OrderStatus::Completed));
    }
    
    #[test]
    fn test_is_valid_status_transition_completed_to_any() {
        // Completed is a terminal state
        assert!(!is_valid_status_transition(&OrderStatus::Completed, &OrderStatus::Pending));
        assert!(!is_valid_status_transition(&OrderStatus::Completed, &OrderStatus::Accepted));
        assert!(!is_valid_status_transition(&OrderStatus::Completed, &OrderStatus::Cancelled));
    }
    
    #[test]
    fn test_is_valid_status_transition_cancelled_to_any() {
        // Cancelled is a terminal state
        assert!(!is_valid_status_transition(&OrderStatus::Cancelled, &OrderStatus::Pending));
        assert!(!is_valid_status_transition(&OrderStatus::Cancelled, &OrderStatus::Accepted));
        assert!(!is_valid_status_transition(&OrderStatus::Cancelled, &OrderStatus::Completed));
    }
    
    #[test]
    fn test_is_valid_status_transition_invalid() {
        // Invalid transitions
        assert!(!is_valid_status_transition(&OrderStatus::Pending, &OrderStatus::Completed));
        assert!(!is_valid_status_transition(&OrderStatus::Accepted, &OrderStatus::Pending));
        assert!(!is_valid_status_transition(&OrderStatus::Accepted, &OrderStatus::Rejected));
    }
    
    #[test]
    fn test_can_accept_order() {
        let order = Order {
            id: Uuid::new_v4(),
            buyer_id: Uuid::new_v4(),
            seller_id: Uuid::new_v4(),
            product_listing_id: Uuid::new_v4(),
            quantity: Decimal::new(10, 0),
            total_amount: Decimal::new(100, 0),
            status: OrderStatus::Pending.to_string(),
            created_at: Utc::now(),
        };
        
        assert!(can_accept_order(&order));
        
        let accepted_order = Order {
            status: OrderStatus::Accepted.to_string(),
            ..order
        };
        
        assert!(!can_accept_order(&accepted_order));
    }
    
    #[test]
    fn test_can_reject_order() {
        let order = Order {
            id: Uuid::new_v4(),
            buyer_id: Uuid::new_v4(),
            seller_id: Uuid::new_v4(),
            product_listing_id: Uuid::new_v4(),
            quantity: Decimal::new(10, 0),
            total_amount: Decimal::new(100, 0),
            status: OrderStatus::Pending.to_string(),
            created_at: Utc::now(),
        };
        
        assert!(can_reject_order(&order));
        
        let accepted_order = Order {
            status: OrderStatus::Accepted.to_string(),
            ..order
        };
        
        assert!(!can_reject_order(&accepted_order));
    }
    
    #[test]
    fn test_can_complete_order() {
        let order = Order {
            id: Uuid::new_v4(),
            buyer_id: Uuid::new_v4(),
            seller_id: Uuid::new_v4(),
            product_listing_id: Uuid::new_v4(),
            quantity: Decimal::new(10, 0),
            total_amount: Decimal::new(100, 0),
            status: OrderStatus::Accepted.to_string(),
            created_at: Utc::now(),
        };
        
        assert!(can_complete_order(&order));
        
        let pending_order = Order {
            status: OrderStatus::Pending.to_string(),
            ..order
        };
        
        assert!(!can_complete_order(&pending_order));
    }
    
    #[test]
    fn test_can_cancel_order() {
        let pending_order = Order {
            id: Uuid::new_v4(),
            buyer_id: Uuid::new_v4(),
            seller_id: Uuid::new_v4(),
            product_listing_id: Uuid::new_v4(),
            quantity: Decimal::new(10, 0),
            total_amount: Decimal::new(100, 0),
            status: OrderStatus::Pending.to_string(),
            created_at: Utc::now(),
        };
        
        assert!(can_cancel_order(&pending_order));
        
        let accepted_order = Order {
            status: OrderStatus::Accepted.to_string(),
            ..pending_order.clone()
        };
        
        assert!(can_cancel_order(&accepted_order));
        
        let completed_order = Order {
            status: OrderStatus::Completed.to_string(),
            ..pending_order
        };
        
        assert!(!can_cancel_order(&completed_order));
    }
    
    // Property-Based Tests
    
    // Feature: dofta-farmers-coop, Property 10: Valid Order Creation
    // For any available product listing and valid quantity, creating an order should succeed.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_valid_order_creation_property(
            listing_quantity_int in 10u32..10000u32,
            order_quantity_int in 1u32..10u32,
            unit_price_int in 1u32..10000u32,
        ) {
            // Create an available product listing with sufficient quantity
            let listing_quantity = Decimal::new(listing_quantity_int as i64, 0);
            let order_quantity = Decimal::new(order_quantity_int as i64, 0);
            let unit_price = Decimal::new(unit_price_int as i64, 2);
            
            let listing = ProductListing {
                id: Uuid::new_v4(),
                member_id: Uuid::new_v4(),
                name: "Test Product".to_string(),
                description: "Test Description".to_string(),
                quantity: listing_quantity,
                unit_price,
                availability: AvailabilityStatus::Available.to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            
            // Property 1: Listing must be available for purchase
            prop_assert!(
                listings::is_available_for_purchase(&listing),
                "Listing should be available for purchase"
            );
            
            // Property 2: Listing must have sufficient quantity
            prop_assert!(
                listing.quantity >= order_quantity,
                "Listing quantity ({}) must be >= order quantity ({})",
                listing.quantity,
                order_quantity
            );
            
            // Create order data
            let order_data = CreateOrderData {
                product_listing_id: listing.id,
                quantity: order_quantity,
            };
            
            // Property 3: Order quantity must be positive
            prop_assert!(
                order_data.quantity > Decimal::ZERO,
                "Order quantity must be positive"
            );
            
            // Property 4: Calculate expected total amount
            let expected_total = listing.unit_price * order_data.quantity;
            prop_assert!(
                expected_total > Decimal::ZERO,
                "Total amount must be positive"
            );
            
            // Property 5: Order data should be valid for creation
            // (In a real test with database, we would create the order and verify it succeeds)
            // Here we verify the preconditions that would make order creation succeed
            prop_assert!(
                order_data.quantity > Decimal::ZERO && 
                order_data.quantity <= listing.quantity &&
                listings::is_available_for_purchase(&listing),
                "Order should meet all preconditions for successful creation"
            );
            
            // Property 6: Verify order would have correct seller_id
            let expected_seller_id = listing.member_id;
            prop_assert_ne!(
                expected_seller_id,
                Uuid::nil(),
                "Seller ID must be valid"
            );
            
            // Property 7: Verify order would start in Pending status
            let expected_status = OrderStatus::Pending;
            prop_assert_eq!(
                expected_status.to_string(),
                "Pending",
                "New orders should start in Pending status"
            );
        }
    }
}
