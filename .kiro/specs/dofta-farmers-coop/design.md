# Design Document: DOFTA Farmers Cooperative

## Overview

The DOFTA system is a web-based cooperative platform built with Rust, providing a secure, performant backend API for organic farmers to trade and collaborate. The architecture follows a layered approach with clear separation between web handlers, business logic, data access, and external services.

**Technology Stack:**
- **Web Framework**: Axum (ergonomic, type-safe, built on Tokio)
- **Database**: PostgreSQL with SQLx (compile-time checked queries)
- **Authentication**: JWT tokens with argon2 password hashing
- **Async Runtime**: Tokio
- **Serialization**: Serde with JSON
- **Testing**: Built-in Rust testing + proptest for property-based testing

## Architecture

The system follows a three-tier architecture:

```
┌─────────────────────────────────────────┐
│         HTTP API Layer (Axum)           │
│  - Route handlers                       │
│  - Request/response serialization       │
│  - Authentication middleware            │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│         Business Logic Layer            │
│  - Domain models                        │
│  - Business rules validation            │
│  - Transaction coordination             │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│         Data Access Layer               │
│  - Repository pattern                   │
│  - Database queries (SQLx)              │
│  - Data persistence                     │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│         PostgreSQL Database             │
└─────────────────────────────────────────┘
```

**Cross-cutting Concerns:**
- Notification service (email via lettre)
- Encryption service (ring for AES encryption)
- Logging (tracing crate)
- Error handling (thiserror for custom errors)

## Components and Interfaces

### 1. Authentication Module

**Responsibilities:**
- User registration and credential storage
- Password hashing and verification
- JWT token generation and validation
- Session management

**Key Types:**
```rust
struct Credentials {
    email: String,
    password: String,
}

struct AuthToken {
    token: String,
    expires_at: DateTime<Utc>,
}

struct Member {
    id: Uuid,
    email: String,
    password_hash: String,
    created_at: DateTime<Utc>,
}
```

**Interface:**
- `register(credentials: Credentials) -> Result<Member, AuthError>`
- `authenticate(credentials: Credentials) -> Result<AuthToken, AuthError>`
- `validate_token(token: &str) -> Result<Uuid, AuthError>`

### 2. Product Listing Module

**Responsibilities:**
- Create, read, update, delete product listings
- Validate product data
- Track availability status

**Key Types:**
```rust
struct ProductListing {
    id: Uuid,
    member_id: Uuid,
    name: String,
    description: String,
    quantity: Decimal,
    unit_price: Decimal,
    availability: AvailabilityStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

enum AvailabilityStatus {
    Available,
    OutOfStock,
    Archived,
}
```

**Interface:**
- `create_listing(member_id: Uuid, data: CreateListingData) -> Result<ProductListing, ListingError>`
- `update_listing(id: Uuid, data: UpdateListingData) -> Result<ProductListing, ListingError>`
- `delete_listing(id: Uuid) -> Result<(), ListingError>`
- `get_listing(id: Uuid) -> Result<ProductListing, ListingError>`

### 3. Order Processing Module

**Responsibilities:**
- Create and manage orders
- Track order status transitions
- Coordinate between buyers and sellers

**Key Types:**
```rust
struct Order {
    id: Uuid,
    buyer_id: Uuid,
    seller_id: Uuid,
    product_listing_id: Uuid,
    quantity: Decimal,
    total_amount: Decimal,
    status: OrderStatus,
    created_at: DateTime<Utc>,
}

enum OrderStatus {
    Pending,
    Accepted,
    Rejected,
    Completed,
    Cancelled,
}
```

**Interface:**
- `create_order(buyer_id: Uuid, listing_id: Uuid, quantity: Decimal) -> Result<Order, OrderError>`
- `accept_order(order_id: Uuid, seller_id: Uuid) -> Result<Order, OrderError>`
- `reject_order(order_id: Uuid, seller_id: Uuid) -> Result<Order, OrderError>`
- `get_order(id: Uuid) -> Result<Order, OrderError>`

### 4. Transaction Module

**Responsibilities:**
- Process payments and financial transactions
- Calculate fees
- Maintain audit trail
- Handle transaction rollbacks

**Key Types:**
```rust
struct Transaction {
    id: Uuid,
    order_id: Uuid,
    amount: Decimal,
    cooperative_fee: Decimal,
    status: TransactionStatus,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Reversed,
}
```

**Interface:**
- `initiate_transaction(order_id: Uuid) -> Result<Transaction, TransactionError>`
- `complete_transaction(transaction_id: Uuid) -> Result<Transaction, TransactionError>`
- `reverse_transaction(transaction_id: Uuid) -> Result<(), TransactionError>`

### 5. Governance Module

**Responsibilities:**
- Create and manage proposals
- Handle voting process
- Calculate voting results
- Enforce one-vote-per-member rule

**Key Types:**
```rust
struct Proposal {
    id: Uuid,
    creator_id: Uuid,
    title: String,
    description: String,
    status: ProposalStatus,
    votes_for: i32,
    votes_against: i32,
    created_at: DateTime<Utc>,
    voting_ends_at: DateTime<Utc>,
}

enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Expired,
}

struct Vote {
    proposal_id: Uuid,
    member_id: Uuid,
    vote_type: VoteType,
    created_at: DateTime<Utc>,
}

enum VoteType {
    For,
    Against,
}
```

**Interface:**
- `create_proposal(creator_id: Uuid, data: CreateProposalData) -> Result<Proposal, GovernanceError>`
- `cast_vote(proposal_id: Uuid, member_id: Uuid, vote_type: VoteType) -> Result<Vote, GovernanceError>`
- `finalize_proposal(proposal_id: Uuid) -> Result<Proposal, GovernanceError>`

### 6. Reputation Module

**Responsibilities:**
- Collect ratings after transactions
- Calculate reputation scores
- Display reputation information

**Key Types:**
```rust
struct Rating {
    id: Uuid,
    transaction_id: Uuid,
    rater_id: Uuid,
    rated_id: Uuid,
    score: i32, // 1-5
    created_at: DateTime<Utc>,
}

struct ReputationScore {
    member_id: Uuid,
    average_rating: Decimal,
    total_ratings: i32,
    total_transactions: i32,
}
```

**Interface:**
- `submit_rating(transaction_id: Uuid, rater_id: Uuid, rated_id: Uuid, score: i32) -> Result<Rating, ReputationError>`
- `get_reputation(member_id: Uuid) -> Result<ReputationScore, ReputationError>`

### 7. Search Module

**Responsibilities:**
- Index product listings
- Execute search queries with filters
- Sort and rank results

**Key Types:**
```rust
struct SearchQuery {
    terms: String,
    filters: SearchFilters,
    sort_by: SortOption,
}

struct SearchFilters {
    category: Option<String>,
    min_price: Option<Decimal>,
    max_price: Option<Decimal>,
    location: Option<String>,
    availability: Option<AvailabilityStatus>,
}

enum SortOption {
    Relevance,
    PriceAsc,
    PriceDesc,
    DateDesc,
}
```

**Interface:**
- `search_listings(query: SearchQuery) -> Result<Vec<ProductListing>, SearchError>`

### 8. Notification Module

**Responsibilities:**
- Send notifications via multiple channels
- Manage notification preferences
- Queue notifications for delivery

**Key Types:**
```rust
struct Notification {
    id: Uuid,
    recipient_id: Uuid,
    notification_type: NotificationType,
    message: String,
    sent_at: Option<DateTime<Utc>>,
}

enum NotificationType {
    OrderPlaced,
    OrderStatusChanged,
    NewProposal,
    VotingEnded,
}

struct NotificationPreferences {
    member_id: Uuid,
    email_enabled: bool,
    in_app_enabled: bool,
}
```

**Interface:**
- `send_notification(recipient_id: Uuid, notification_type: NotificationType, message: String) -> Result<(), NotificationError>`
- `update_preferences(member_id: Uuid, preferences: NotificationPreferences) -> Result<(), NotificationError>`

### 9. Security Module

**Responsibilities:**
- Encrypt sensitive data
- Manage encryption keys
- Audit data access
- Handle data export and deletion requests

**Interface:**
- `encrypt_data(plaintext: &[u8]) -> Result<Vec<u8>, SecurityError>`
- `decrypt_data(ciphertext: &[u8]) -> Result<Vec<u8>, SecurityError>`
- `log_data_access(member_id: Uuid, resource: String, action: String) -> Result<(), SecurityError>`
- `export_member_data(member_id: Uuid) -> Result<MemberDataExport, SecurityError>`
- `delete_member_data(member_id: Uuid) -> Result<(), SecurityError>`

### 10. Reporting Module

**Responsibilities:**
- Generate sales and purchase reports
- Calculate analytics metrics
- Export reports in multiple formats

**Key Types:**
```rust
struct SalesReport {
    member_id: Uuid,
    period_start: DateTime<Utc>,
    period_end: DateTime<Utc>,
    total_sales: Decimal,
    transaction_count: i32,
    transactions: Vec<Transaction>,
}

struct ReportFormat {
    format_type: FormatType,
}

enum FormatType {
    CSV,
    PDF,
}
```

**Interface:**
- `generate_sales_report(member_id: Uuid, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<SalesReport, ReportError>`
- `generate_purchase_report(member_id: Uuid, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<PurchaseReport, ReportError>`
- `export_report(report: Report, format: FormatType) -> Result<Vec<u8>, ReportError>`

## Data Models

### Database Schema

**members table:**
```sql
CREATE TABLE members (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

**product_listings table:**
```sql
CREATE TABLE product_listings (
    id UUID PRIMARY KEY,
    member_id UUID NOT NULL REFERENCES members(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    quantity DECIMAL(10,2) NOT NULL,
    unit_price DECIMAL(10,2) NOT NULL,
    availability VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

**orders table:**
```sql
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    buyer_id UUID NOT NULL REFERENCES members(id),
    seller_id UUID NOT NULL REFERENCES members(id),
    product_listing_id UUID NOT NULL REFERENCES product_listings(id),
    quantity DECIMAL(10,2) NOT NULL,
    total_amount DECIMAL(10,2) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

**transactions table:**
```sql
CREATE TABLE transactions (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL REFERENCES orders(id),
    amount DECIMAL(10,2) NOT NULL,
    cooperative_fee DECIMAL(10,2) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP
);
```

**proposals table:**
```sql
CREATE TABLE proposals (
    id UUID PRIMARY KEY,
    creator_id UUID NOT NULL REFERENCES members(id),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(50) NOT NULL,
    votes_for INTEGER NOT NULL DEFAULT 0,
    votes_against INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    voting_ends_at TIMESTAMP NOT NULL
);
```

**votes table:**
```sql
CREATE TABLE votes (
    proposal_id UUID NOT NULL REFERENCES proposals(id),
    member_id UUID NOT NULL REFERENCES members(id),
    vote_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (proposal_id, member_id)
);
```

**ratings table:**
```sql
CREATE TABLE ratings (
    id UUID PRIMARY KEY,
    transaction_id UUID NOT NULL REFERENCES transactions(id),
    rater_id UUID NOT NULL REFERENCES members(id),
    rated_id UUID NOT NULL REFERENCES members(id),
    score INTEGER NOT NULL CHECK (score >= 1 AND score <= 5),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

**notifications table:**
```sql
CREATE TABLE notifications (
    id UUID PRIMARY KEY,
    recipient_id UUID NOT NULL REFERENCES members(id),
    notification_type VARCHAR(100) NOT NULL,
    message TEXT NOT NULL,
    sent_at TIMESTAMP
);
```

**notification_preferences table:**
```sql
CREATE TABLE notification_preferences (
    member_id UUID PRIMARY KEY REFERENCES members(id),
    email_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    in_app_enabled BOOLEAN NOT NULL DEFAULT TRUE
);
```

**audit_log table:**
```sql
CREATE TABLE audit_log (
    id UUID PRIMARY KEY,
    member_id UUID REFERENCES members(id),
    resource VARCHAR(255) NOT NULL,
    action VARCHAR(100) NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW()
);
```


## Correctness Properties

A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.

### Authentication and Registration Properties

**Property 1: Registration-Authentication Round Trip**
*For any* valid registration credentials, registering a member and then authenticating with those same credentials should succeed and return a valid auth token.
**Validates: Requirements 1.1, 1.3**

**Property 2: Invalid Registration Rejection**
*For any* incomplete or invalid registration data (missing email, invalid email format, weak password), the system should reject registration and provide specific error messages.
**Validates: Requirements 1.2**

**Property 3: Invalid Authentication Rejection**
*For any* invalid credentials (wrong password, non-existent email), authentication should fail and the failure should be logged.
**Validates: Requirements 1.4**

**Property 4: Password Encryption Invariant**
*For any* registered member, the stored password hash should never equal the plaintext password, and password verification should succeed for the correct password.
**Validates: Requirements 1.5**

### Product Listing Properties

**Property 5: Listing Creation and Retrieval**
*For any* valid product listing data, creating a listing should result in a listing that can be retrieved with all fields matching the input data.
**Validates: Requirements 2.1**

**Property 6: Invalid Listing Rejection**
*For any* invalid listing data (negative price, negative quantity, empty name), the system should reject the listing with appropriate validation errors.
**Validates: Requirements 2.2**

**Property 7: Listing Update Preserves Identity**
*For any* existing listing and valid update data, updating the listing should preserve the listing ID and member ID while updating the specified fields and timestamp.
**Validates: Requirements 2.3**

**Property 8: Listing Deletion Removes Visibility**
*For any* existing listing, deleting it should make it unavailable in search results and public queries.
**Validates: Requirements 2.4**

**Property 9: Listing Completeness Invariant**
*For any* created listing, it must contain non-empty name, description, positive quantity, positive unit price, and a valid availability status.
**Validates: Requirements 2.5**

### Order Processing Properties

**Property 10: Valid Order Creation**
*For any* available product listing and valid quantity, creating an order should succeed and notify the seller.
**Validates: Requirements 3.1**

**Property 11: Unavailable Product Order Rejection**
*For any* unavailable or out-of-stock product listing, attempting to create an order should fail with an appropriate error.
**Validates: Requirements 3.2**

**Property 12: Order Status Transitions**
*For any* order, status transitions should follow valid paths: Pending → Accepted/Rejected, Accepted → Completed, and invalid transitions should be rejected.
**Validates: Requirements 3.3, 3.4, 3.5**

### Transaction Properties

**Property 13: Transaction Amount Calculation**
*For any* accepted order, the transaction amount should equal the order quantity multiplied by unit price plus the cooperative fee percentage.
**Validates: Requirements 4.1**

**Property 14: Payment Confirmation Updates**
*For any* pending transaction, confirming payment should update the status to Completed and trigger notifications to both buyer and seller.
**Validates: Requirements 4.2**

**Property 15: Transaction Completion Side Effects**
*For any* completed transaction, the product inventory should decrease by the order quantity and member balances should be updated accordingly.
**Validates: Requirements 4.3**

**Property 16: Transaction Rollback on Failure**
*For any* transaction that fails during processing, all partial changes (inventory, balances) should be reversed and both parties notified.
**Validates: Requirements 4.4**

**Property 17: Transaction Audit Trail**
*For any* transaction, there should exist corresponding audit log entries recording the transaction creation, status changes, and completion.
**Validates: Requirements 4.5**

### Governance Properties

**Property 18: Proposal Visibility**
*For any* created proposal, it should be visible to all members and have an initial status of Active.
**Validates: Requirements 5.1**

**Property 19: Vote Recording and Counting**
*For any* active proposal and member vote, casting a vote should increment the appropriate vote count (for/against) by exactly one.
**Validates: Requirements 5.2**

**Property 20: Proposal Finalization**
*For any* proposal past its voting deadline, finalizing it should set the status to Passed if votes_for > votes_against, otherwise Rejected.
**Validates: Requirements 5.3**

**Property 21: One Vote Per Member Invariant**
*For any* proposal and member, attempting to vote multiple times should result in only one recorded vote (the first or last, depending on policy).
**Validates: Requirements 5.4**

**Property 22: Active Proposal Vote Visibility**
*For any* active proposal, the current vote counts should be visible to all members.
**Validates: Requirements 5.5**

### Reputation Properties

**Property 23: Post-Transaction Rating Capability**
*For any* completed transaction, both the buyer and seller should be able to submit ratings for each other.
**Validates: Requirements 6.1**

**Property 24: Rating Updates Reputation Score**
*For any* submitted rating, the recipient's reputation score should be recalculated as the average of all their received ratings.
**Validates: Requirements 6.2, 6.3**

**Property 25: Reputation Display Completeness**
*For any* member profile or product listing, the displayed information should include the reputation score and total transaction count.
**Validates: Requirements 6.4, 6.5**

### Search Properties

**Property 26: Search Term Matching**
*For any* search query with terms, all returned listings should contain at least one of the search terms in their name or description.
**Validates: Requirements 7.1**

**Property 27: Filter Application**
*For any* search with filters (price range, category, location, availability), all returned listings should satisfy all applied filter criteria.
**Validates: Requirements 7.2, 7.3**

**Property 28: Search Result Sorting**
*For any* search with a sort option, results should be ordered according to the specified criteria (price ascending/descending, date, relevance).
**Validates: Requirements 7.4**

### Notification Properties

**Property 29: Event-Triggered Notifications**
*For any* significant event (order placed, order status changed, proposal created), the system should create and send notifications to the relevant members.
**Validates: Requirements 8.1, 8.2, 8.3**

**Property 30: Notification Preference Enforcement**
*For any* member with email notifications enabled, notifications should be sent via email; if disabled, only in-app notifications should be created.
**Validates: Requirements 8.4**

**Property 31: Notification Preference Persistence**
*For any* member, updating notification preferences should persist the changes and affect future notifications accordingly.
**Validates: Requirements 8.5**

### Security Properties

**Property 32: Sensitive Data Encryption**
*For any* sensitive data (passwords, payment info), the stored representation should be encrypted and not equal to the plaintext.
**Validates: Requirements 9.1**

**Property 33: Access Control Enforcement**
*For any* member attempting to access another member's private data, the access should be denied unless explicitly authorized.
**Validates: Requirements 9.2**

**Property 34: Data Export Completeness**
*For any* member requesting data export, the export should include all their personal data, listings, orders, transactions, and ratings.
**Validates: Requirements 9.3**

**Property 35: Data Deletion Preserves Transactions**
*For any* member requesting data deletion, their personal information should be removed but transaction records should remain (anonymized).
**Validates: Requirements 9.4**

**Property 36: Audit Logging Invariant**
*For any* access to sensitive data, an audit log entry should be created with timestamp, member ID, resource, and action.
**Validates: Requirements 9.5**

### Reporting Properties

**Property 37: Report Data Accuracy**
*For any* report request (sales or purchases) with a date range, the report should include all and only transactions within that date range for the requesting member.
**Validates: Requirements 10.1, 10.2**

**Property 38: Report Metrics Calculation**
*For any* generated report, the calculated metrics (total sales, total purchases, average transaction value) should match the sum and average of the included transactions.
**Validates: Requirements 10.3**

**Property 39: Report Export Format**
*For any* report exported in CSV or PDF format, the output should be valid and parseable in the specified format.
**Validates: Requirements 10.4**

**Property 40: Administrative Analytics Access**
*For any* member with administrative privileges, they should be able to access cooperative-wide analytics; non-admin members should be denied access.
**Validates: Requirements 10.5**

## Error Handling

The system uses Rust's Result type for error handling, with custom error types for each module:

**Error Type Hierarchy:**
```rust
#[derive(Debug, thiserror::Error)]
enum DoftaError {
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
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Internal error: {0}")]
    Internal(String),
}
```

**Error Handling Principles:**
1. All errors are typed and descriptive
2. Database errors are wrapped and logged
3. User-facing errors exclude sensitive information
4. All errors are logged with appropriate severity
5. Transaction failures trigger automatic rollback

**HTTP Error Mapping:**
- `AuthError` → 401 Unauthorized or 403 Forbidden
- `ValidationError` → 400 Bad Request
- `NotFoundError` → 404 Not Found
- `ConflictError` → 409 Conflict
- `InternalError` → 500 Internal Server Error

## Testing Strategy

The DOFTA system will use a comprehensive testing approach combining unit tests and property-based tests to ensure correctness.

### Property-Based Testing

**Framework**: proptest (Rust's property-based testing library)

**Configuration**:
- Minimum 100 iterations per property test
- Each test tagged with feature name and property number
- Tag format: `// Feature: dofta-farmers-coop, Property N: [property description]`

**Test Organization**:
- Property tests co-located with implementation in module test submodules
- Each correctness property from the design document implemented as a property test
- Generators for domain types (Member, ProductListing, Order, etc.)

**Example Property Test Structure**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    // Feature: dofta-farmers-coop, Property 1: Registration-Authentication Round Trip
    proptest! {
        #[test]
        fn test_registration_auth_roundtrip(
            email in "[a-z]{5,10}@[a-z]{5,10}\\.(com|org)",
            password in "[A-Za-z0-9]{8,20}"
        ) {
            // Test implementation
        }
    }
}
```

### Unit Testing

**Purpose**: Test specific examples, edge cases, and integration points

**Coverage Areas**:
- Edge cases (empty inputs, boundary values, maximum sizes)
- Error conditions (invalid data, unauthorized access, conflicts)
- Integration between modules
- Database query correctness
- API endpoint behavior

**Test Organization**:
- Unit tests in same files as property tests
- Integration tests in `tests/` directory
- Database tests use test containers or in-memory SQLite

### Testing Balance

- Property tests verify universal correctness across many inputs
- Unit tests verify specific examples and edge cases
- Both are necessary and complementary
- Property tests catch unexpected bugs through randomization
- Unit tests provide concrete examples and regression prevention

### Test Execution

**Commands**:
- `cargo test` - Run all tests
- `cargo test --test integration` - Run integration tests only
- `cargo test property` - Run property tests only

**CI/CD Integration**:
- All tests run on every commit
- Property tests run with increased iterations (1000+) in CI
- Code coverage tracked and reported
- No merge without passing tests
