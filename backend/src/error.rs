use thiserror::Error;

/// Main error type for the DOFTA system
#[derive(Debug, Error)]
pub enum DoftaError {
    #[error("Authentication error: {0}")]
    Auth(#[from] AuthError),
    
    #[error("Listing error: {0}")]
    Listing(#[from] ListingError),
    
    #[error("Order error: {0}")]
    Order(#[from] OrderError),
    
    #[error("Transaction error: {0}")]
    Transaction(#[from] TransactionError),
    
    #[error("Governance error: {0}")]
    Governance(#[from] GovernanceError),
    
    #[error("Reputation error: {0}")]
    Reputation(#[from] ReputationError),
    
    #[error("Search error: {0}")]
    Search(#[from] SearchError),
    
    #[error("Notification error: {0}")]
    Notification(#[from] NotificationError),
    
    #[error("Security error: {0}")]
    Security(#[from] SecurityError),
    
    #[error("Report error: {0}")]
    Report(#[from] ReportError),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Authentication module errors
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Registration failed: {0}")]
    RegistrationFailed(String),
    
    #[error("Password hashing failed")]
    HashingFailed,
    
    #[error("Member not found")]
    MemberNotFound,
}

/// Product listing module errors
#[derive(Debug, Error)]
pub enum ListingError {
    #[error("Invalid listing data: {0}")]
    InvalidData(String),
    
    #[error("Listing not found")]
    NotFound,
    
    #[error("Unauthorized access")]
    Unauthorized,
    
    #[error("Listing already exists")]
    AlreadyExists,
}

/// Order processing module errors
#[derive(Debug, Error)]
pub enum OrderError {
    #[error("Invalid order data: {0}")]
    InvalidData(String),
    
    #[error("Order not found")]
    NotFound,
    
    #[error("Product unavailable")]
    ProductUnavailable,
    
    #[error("Insufficient quantity")]
    InsufficientQuantity,
    
    #[error("Invalid status transition: {0}")]
    InvalidStatusTransition(String),
    
    #[error("Unauthorized access")]
    Unauthorized,
}

/// Transaction module errors
#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Transaction not found")]
    NotFound,
    
    #[error("Transaction failed: {0}")]
    Failed(String),
    
    #[error("Invalid amount")]
    InvalidAmount,
    
    #[error("Rollback failed: {0}")]
    RollbackFailed(String),
}

/// Governance module errors
#[derive(Debug, Error)]
pub enum GovernanceError {
    #[error("Proposal not found")]
    ProposalNotFound,
    
    #[error("Invalid proposal data: {0}")]
    InvalidData(String),
    
    #[error("Voting period ended")]
    VotingEnded,
    
    #[error("Already voted")]
    AlreadyVoted,
    
    #[error("Unauthorized access")]
    Unauthorized,
}

/// Reputation module errors
#[derive(Debug, Error)]
pub enum ReputationError {
    #[error("Rating not found")]
    NotFound,
    
    #[error("Invalid rating: {0}")]
    InvalidRating(String),
    
    #[error("Transaction not completed")]
    TransactionNotCompleted,
    
    #[error("Already rated")]
    AlreadyRated,
}

/// Search module errors
#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Invalid search query: {0}")]
    InvalidQuery(String),
    
    #[error("Search failed: {0}")]
    Failed(String),
}

/// Notification module errors
#[derive(Debug, Error)]
pub enum NotificationError {
    #[error("Failed to send notification: {0}")]
    SendFailed(String),
    
    #[error("Invalid notification type")]
    InvalidType,
    
    #[error("Recipient not found")]
    RecipientNotFound,
}

/// Security module errors
#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    
    #[error("Access denied")]
    AccessDenied,
    
    #[error("Data export failed: {0}")]
    ExportFailed(String),
    
    #[error("Data deletion failed: {0}")]
    DeletionFailed(String),
}

/// Reporting module errors
#[derive(Debug, Error)]
pub enum ReportError {
    #[error("Report generation failed: {0}")]
    GenerationFailed(String),
    
    #[error("Invalid date range")]
    InvalidDateRange,
    
    #[error("Export failed: {0}")]
    ExportFailed(String),
    
    #[error("Unauthorized access")]
    Unauthorized,
}

/// Result type alias for DOFTA operations
pub type Result<T> = std::result::Result<T, DoftaError>;
