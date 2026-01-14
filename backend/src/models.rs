use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Member represents a registered farmer in the cooperative
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Member {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl Member {
    /// Validate member data
    pub fn validate(&self) -> Result<(), String> {
        if self.email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }
        
        if !self.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        
        if self.password_hash.is_empty() {
            return Err("Password hash cannot be empty".to_string());
        }
        
        Ok(())
    }
}

/// Availability status for product listings
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum AvailabilityStatus {
    Available,
    OutOfStock,
    Archived,
}

impl std::fmt::Display for AvailabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AvailabilityStatus::Available => write!(f, "Available"),
            AvailabilityStatus::OutOfStock => write!(f, "OutOfStock"),
            AvailabilityStatus::Archived => write!(f, "Archived"),
        }
    }
}

impl std::str::FromStr for AvailabilityStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Available" => Ok(AvailabilityStatus::Available),
            "OutOfStock" => Ok(AvailabilityStatus::OutOfStock),
            "Archived" => Ok(AvailabilityStatus::Archived),
            _ => Err(format!("Invalid availability status: {}", s)),
        }
    }
}

/// Product listing represents an item offered for sale
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductListing {
    pub id: Uuid,
    pub member_id: Uuid,
    pub name: String,
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub availability: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ProductListing {
    /// Validate product listing data
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Product name cannot be empty".to_string());
        }
        
        if self.description.trim().is_empty() {
            return Err("Product description cannot be empty".to_string());
        }
        
        if self.quantity <= Decimal::ZERO {
            return Err("Quantity must be positive".to_string());
        }
        
        if self.unit_price <= Decimal::ZERO {
            return Err("Unit price must be positive".to_string());
        }
        
        // Validate availability status
        self.availability.parse::<AvailabilityStatus>()
            .map_err(|e| format!("Invalid availability status: {}", e))?;
        
        Ok(())
    }
}

/// Order status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum OrderStatus {
    Pending,
    Accepted,
    Rejected,
    Completed,
    Cancelled,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "Pending"),
            OrderStatus::Accepted => write!(f, "Accepted"),
            OrderStatus::Rejected => write!(f, "Rejected"),
            OrderStatus::Completed => write!(f, "Completed"),
            OrderStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl std::str::FromStr for OrderStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(OrderStatus::Pending),
            "Accepted" => Ok(OrderStatus::Accepted),
            "Rejected" => Ok(OrderStatus::Rejected),
            "Completed" => Ok(OrderStatus::Completed),
            "Cancelled" => Ok(OrderStatus::Cancelled),
            _ => Err(format!("Invalid order status: {}", s)),
        }
    }
}

/// Order represents a purchase request
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub product_listing_id: Uuid,
    pub quantity: Decimal,
    pub total_amount: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl Order {
    /// Validate order data
    pub fn validate(&self) -> Result<(), String> {
        if self.quantity <= Decimal::ZERO {
            return Err("Order quantity must be positive".to_string());
        }
        
        if self.total_amount <= Decimal::ZERO {
            return Err("Total amount must be positive".to_string());
        }
        
        // Validate status
        self.status.parse::<OrderStatus>()
            .map_err(|e| format!("Invalid order status: {}", e))?;
        
        Ok(())
    }
}

/// Transaction status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Reversed,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Pending => write!(f, "Pending"),
            TransactionStatus::Completed => write!(f, "Completed"),
            TransactionStatus::Failed => write!(f, "Failed"),
            TransactionStatus::Reversed => write!(f, "Reversed"),
        }
    }
}

impl std::str::FromStr for TransactionStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(TransactionStatus::Pending),
            "Completed" => Ok(TransactionStatus::Completed),
            "Failed" => Ok(TransactionStatus::Failed),
            "Reversed" => Ok(TransactionStatus::Reversed),
            _ => Err(format!("Invalid transaction status: {}", s)),
        }
    }
}

/// Transaction represents a financial exchange
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub order_id: Uuid,
    pub amount: Decimal,
    pub cooperative_fee: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Transaction {
    /// Validate transaction data
    pub fn validate(&self) -> Result<(), String> {
        if self.amount <= Decimal::ZERO {
            return Err("Transaction amount must be positive".to_string());
        }
        
        if self.cooperative_fee < Decimal::ZERO {
            return Err("Cooperative fee cannot be negative".to_string());
        }
        
        // Validate status
        self.status.parse::<TransactionStatus>()
            .map_err(|e| format!("Invalid transaction status: {}", e))?;
        
        Ok(())
    }
}

/// Proposal status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Expired,
}

impl std::fmt::Display for ProposalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalStatus::Active => write!(f, "Active"),
            ProposalStatus::Passed => write!(f, "Passed"),
            ProposalStatus::Rejected => write!(f, "Rejected"),
            ProposalStatus::Expired => write!(f, "Expired"),
        }
    }
}

impl std::str::FromStr for ProposalStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(ProposalStatus::Active),
            "Passed" => Ok(ProposalStatus::Passed),
            "Rejected" => Ok(ProposalStatus::Rejected),
            "Expired" => Ok(ProposalStatus::Expired),
            _ => Err(format!("Invalid proposal status: {}", s)),
        }
    }
}

/// Proposal represents a governance decision
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Proposal {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub votes_for: i32,
    pub votes_against: i32,
    pub created_at: DateTime<Utc>,
    pub voting_ends_at: DateTime<Utc>,
}

impl Proposal {
    /// Validate proposal data
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Proposal title cannot be empty".to_string());
        }
        
        if self.description.trim().is_empty() {
            return Err("Proposal description cannot be empty".to_string());
        }
        
        if self.votes_for < 0 {
            return Err("Votes for cannot be negative".to_string());
        }
        
        if self.votes_against < 0 {
            return Err("Votes against cannot be negative".to_string());
        }
        
        // Validate status
        self.status.parse::<ProposalStatus>()
            .map_err(|e| format!("Invalid proposal status: {}", e))?;
        
        Ok(())
    }
}

/// Vote type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum VoteType {
    For,
    Against,
}

impl std::fmt::Display for VoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoteType::For => write!(f, "For"),
            VoteType::Against => write!(f, "Against"),
        }
    }
}

impl std::str::FromStr for VoteType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "For" => Ok(VoteType::For),
            "Against" => Ok(VoteType::Against),
            _ => Err(format!("Invalid vote type: {}", s)),
        }
    }
}

/// Vote represents a member's vote on a proposal
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Vote {
    pub proposal_id: Uuid,
    pub member_id: Uuid,
    pub vote_type: String,
    pub created_at: DateTime<Utc>,
}

impl Vote {
    /// Validate vote data
    pub fn validate(&self) -> Result<(), String> {
        // Validate vote type
        self.vote_type.parse::<VoteType>()
            .map_err(|e| format!("Invalid vote type: {}", e))?;
        
        Ok(())
    }
}

/// Rating represents a member's rating of another member
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Rating {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub rater_id: Uuid,
    pub rated_id: Uuid,
    pub score: i32,
    pub created_at: DateTime<Utc>,
}

impl Rating {
    /// Validate rating data
    pub fn validate(&self) -> Result<(), String> {
        if self.score < 1 || self.score > 5 {
            return Err("Rating score must be between 1 and 5".to_string());
        }
        
        Ok(())
    }
}

/// Notification type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum NotificationType {
    OrderPlaced,
    OrderStatusChanged,
    NewProposal,
    VotingEnded,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::OrderPlaced => write!(f, "OrderPlaced"),
            NotificationType::OrderStatusChanged => write!(f, "OrderStatusChanged"),
            NotificationType::NewProposal => write!(f, "NewProposal"),
            NotificationType::VotingEnded => write!(f, "VotingEnded"),
        }
    }
}

impl std::str::FromStr for NotificationType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OrderPlaced" => Ok(NotificationType::OrderPlaced),
            "OrderStatusChanged" => Ok(NotificationType::OrderStatusChanged),
            "NewProposal" => Ok(NotificationType::NewProposal),
            "VotingEnded" => Ok(NotificationType::VotingEnded),
            _ => Err(format!("Invalid notification type: {}", s)),
        }
    }
}

/// Notification represents a message to a member
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub recipient_id: Uuid,
    pub notification_type: String,
    pub message: String,
    pub sent_at: Option<DateTime<Utc>>,
}

impl Notification {
    /// Validate notification data
    pub fn validate(&self) -> Result<(), String> {
        if self.message.trim().is_empty() {
            return Err("Notification message cannot be empty".to_string());
        }
        
        // Validate notification type
        self.notification_type.parse::<NotificationType>()
            .map_err(|e| format!("Invalid notification type: {}", e))?;
        
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use rust_decimal::Decimal;
    
    // Feature: dofta-farmers-coop, Property 9: Listing Completeness Invariant
    // For any created listing, it must contain non-empty name, description, 
    // positive quantity, positive unit price, and a valid availability status.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        
        #[test]
        fn test_listing_completeness_invariant(
            name in "[a-zA-Z ]{1,50}",
            description in "[a-zA-Z0-9 .,]{1,200}",
            quantity_int in 1u32..10000u32,
            quantity_frac in 0u32..100u32,
            price_int in 1u32..10000u32,
            price_frac in 0u32..100u32,
            availability_idx in 0usize..3usize,
        ) {
            let availability_options = ["Available", "OutOfStock", "Archived"];
            let availability = availability_options[availability_idx].to_string();
            
            let quantity = Decimal::new((quantity_int * 100 + quantity_frac) as i64, 2);
            let unit_price = Decimal::new((price_int * 100 + price_frac) as i64, 2);
            
            let listing = ProductListing {
                id: Uuid::new_v4(),
                member_id: Uuid::new_v4(),
                name: name.clone(),
                description: description.clone(),
                quantity,
                unit_price,
                availability: availability.clone(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            
            // Validate the listing
            let validation_result = listing.validate();
            
            // Property: All fields must be valid
            prop_assert!(validation_result.is_ok(), "Listing validation failed: {:?}", validation_result);
            
            // Property: Name must be non-empty
            prop_assert!(!listing.name.trim().is_empty(), "Name is empty");
            
            // Property: Description must be non-empty
            prop_assert!(!listing.description.trim().is_empty(), "Description is empty");
            
            // Property: Quantity must be positive
            prop_assert!(listing.quantity > Decimal::ZERO, "Quantity is not positive");
            
            // Property: Unit price must be positive
            prop_assert!(listing.unit_price > Decimal::ZERO, "Unit price is not positive");
            
            // Property: Availability must be valid
            let availability_status = listing.availability.parse::<AvailabilityStatus>();
            prop_assert!(availability_status.is_ok(), "Invalid availability status");
        }
    }
}
