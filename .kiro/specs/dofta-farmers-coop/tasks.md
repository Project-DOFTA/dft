# Implementation Plan: DOFTA Farmers Cooperative

## Overview

This implementation plan breaks down the DOFTA cooperative platform into incremental coding tasks. Each task builds on previous work, with property-based tests integrated throughout to validate correctness early. The plan follows a bottom-up approach: core data models → business logic → API layer → integration.

## Tasks

- [x] 1. Project setup and infrastructure
  - Initialize Cargo workspace with proper dependencies
  - Set up database connection pool and migrations
  - Configure logging and error handling
  - Create base error types and Result aliases
  - _Requirements: All (foundational)_

- [x] 2. Implement core data models and database schema
  - [x] 2.1 Create database migration files for all tables
    - Write SQL migrations for members, product_listings, orders, transactions, proposals, votes, ratings, notifications, audit_log tables
    - _Requirements: 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 8.1, 9.5, 10.1_

  - [x] 2.2 Implement Rust domain models with Serde serialization
    - Create structs for Member, ProductListing, Order, Transaction, Proposal, Vote, Rating, Notification
    - Add validation methods to each model
    - _Requirements: 1.1, 2.1, 3.1, 4.1, 5.1, 6.1_

  - [x] 2.3 Write property test for model validation
    - **Property 9: Listing Completeness Invariant**
    - **Validates: Requirements 2.5**

- [ ] 3. Implement authentication module
  - [x] 3.1 Create password hashing and verification functions
    - Use argon2 for password hashing
    - Implement hash and verify functions
    - _Requirements: 1.5_

  - [x] 3.2 Write property test for password encryption
    - **Property 4: Password Encryption Invariant**
    - **Validates: Requirements 1.5**

  - [x] 3.3 Implement member registration logic
    - Create registration function with validation
    - Store member in database with hashed password
    - _Requirements: 1.1, 1.2_

  - [x] 3.4 Write property test for registration validation

    - **Property 2: Invalid Registration Rejection**
    - **Validates: Requirements 1.2**

  - [x] 3.5 Implement JWT token generation and validation
    - Create token generation with expiration
    - Implement token validation middleware
    - _Requirements: 1.3, 1.4_

  - [x]* 3.6 Write property test for registration-authentication round trip
    - **Property 1: Registration-Authentication Round Trip**
    - **Validates: Requirements 1.1, 1.3**

  - [x]* 3.7 Write property test for invalid authentication
    - **Property 3: Invalid Authentication Rejection**
    - **Validates: Requirements 1.4**

- [ ] 4. Checkpoint - Ensure authentication tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 5. Implement product listing module
  - [x] 5.1 Create repository layer for product listings
    - Implement CRUD operations with SQLx
    - Add query methods for search and filtering
    - _Requirements: 2.1, 2.2, 2.3, 2.4_

  - [x] 5.2 Implement listing business logic
    - Add validation for listing creation and updates
    - Implement availability status management
    - _Requirements: 2.1, 2.2, 2.3_

  - [x] 5.3 Write property test for listing creation and retrieval


    - **Property 5: Listing Creation and Retrieval**
    - **Validates: Requirements 2.1**
  
  - [x] 5.4 Write property test for invalid listing rejection

    - **Property 6: Invalid Listing Rejection**
    - **Validates: Requirements 2.2**

  - [x] 5.5 Write property test for listing updates

    - **Property 7: Listing Update Preserves Identity**
    - **Validates: Requirements 2.3**

  - [x] 5.6 Write property test for listing deletion

    - **Property 8: Listing Deletion Removes Visibility**
    - **Validates: Requirements 2.4**

- [x] 6. Implement order processing module
  - [x] 6.1 Create repository layer for orders
    - Implement order CRUD operations
    - Add status transition methods
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5_

  - [x] 6.2 Implement order business logic
    - Add validation for order creation (check availability)
    - Implement order acceptance and rejection
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

  - [ ] 6.3 Write property test for valid order creation

    - **Property 10: Valid Order Creation**
    - **Validates: Requirements 3.1**

  - [ ]* 6.4 Write property test for unavailable product rejection
    - **Property 11: Unavailable Product Order Rejection**
    - **Validates: Requirements 3.2**

  - [ ]* 6.5 Write property test for order status transitions
    - **Property 12: Order Status Transitions**
    - **Validates: Requirements 3.3, 3.4, 3.5**

- [ ] 7. Checkpoint - Ensure order processing tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 8. Implement transaction module
  - [ ] 8.1 Create repository layer for transactions
    - Implement transaction CRUD operations
    - Add audit trail logging
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

  - [ ] 8.2 Implement transaction business logic with rollback
    - Add amount calculation with cooperative fees
    - Implement transaction completion with inventory updates
    - Add rollback mechanism for failed transactions
    - _Requirements: 4.1, 4.2, 4.3, 4.4_

  - [ ]* 8.3 Write property test for transaction amount calculation
    - **Property 13: Transaction Amount Calculation**
    - **Validates: Requirements 4.1**

  - [ ]* 8.4 Write property test for payment confirmation
    - **Property 14: Payment Confirmation Updates**
    - **Validates: Requirements 4.2**

  - [ ]* 8.5 Write property test for transaction completion side effects
    - **Property 15: Transaction Completion Side Effects**
    - **Validates: Requirements 4.3**

  - [ ]* 8.6 Write property test for transaction rollback
    - **Property 16: Transaction Rollback on Failure**
    - **Validates: Requirements 4.4**

  - [ ]* 8.7 Write property test for audit trail
    - **Property 17: Transaction Audit Trail**
    - **Validates: Requirements 4.5**

- [ ] 9. Implement governance module
  - [ ] 9.1 Create repository layer for proposals and votes
    - Implement proposal CRUD operations
    - Add vote recording with uniqueness constraint
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

  - [ ] 9.2 Implement governance business logic
    - Add proposal creation and finalization
    - Implement vote casting with duplicate prevention
    - Add vote counting and result calculation
    - _Requirements: 5.1, 5.2, 5.3, 5.4_

  - [ ]* 9.3 Write property test for proposal visibility
    - **Property 18: Proposal Visibility**
    - **Validates: Requirements 5.1**

  - [ ]* 9.4 Write property test for vote recording
    - **Property 19: Vote Recording and Counting**
    - **Validates: Requirements 5.2**

  - [ ]* 9.5 Write property test for proposal finalization
    - **Property 20: Proposal Finalization**
    - **Validates: Requirements 5.3**

  - [ ]* 9.6 Write property test for one vote per member
    - **Property 21: One Vote Per Member Invariant**
    - **Validates: Requirements 5.4**

  - [ ]* 9.7 Write property test for vote visibility
    - **Property 22: Active Proposal Vote Visibility**
    - **Validates: Requirements 5.5**

- [ ] 10. Checkpoint - Ensure governance tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 11. Implement reputation module
  - [ ] 11.1 Create repository layer for ratings
    - Implement rating CRUD operations
    - Add reputation score calculation queries
    - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5_

  - [ ] 11.2 Implement reputation business logic
    - Add rating submission after transaction completion
    - Implement reputation score calculation
    - _Requirements: 6.1, 6.2, 6.3_

  - [ ]* 11.3 Write property test for post-transaction rating
    - **Property 23: Post-Transaction Rating Capability**
    - **Validates: Requirements 6.1**

  - [ ]* 11.4 Write property test for reputation score updates
    - **Property 24: Rating Updates Reputation Score**
    - **Validates: Requirements 6.2, 6.3**

  - [ ]* 11.5 Write property test for reputation display
    - **Property 25: Reputation Display Completeness**
    - **Validates: Requirements 6.4, 6.5**

- [ ] 12. Implement search module
  - [ ] 12.1 Create search repository with filtering
    - Implement full-text search queries
    - Add filter support for category, price, location, availability
    - Add sorting options
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

  - [ ] 12.2 Implement search business logic
    - Add search term parsing and matching
    - Implement filter application
    - Add result sorting
    - _Requirements: 7.1, 7.2, 7.4_

  - [ ]* 12.3 Write property test for search term matching
    - **Property 26: Search Term Matching**
    - **Validates: Requirements 7.1**

  - [ ]* 12.4 Write property test for filter application
    - **Property 27: Filter Application**
    - **Validates: Requirements 7.2, 7.3**

  - [ ]* 12.5 Write property test for search result sorting
    - **Property 28: Search Result Sorting**
    - **Validates: Requirements 7.4**

  - [ ]* 12.6 Write unit test for empty search results
    - Test that empty results return helpful message
    - _Requirements: 7.5_

- [ ] 13. Implement notification module
  - [ ] 13.1 Create repository layer for notifications
    - Implement notification CRUD operations
    - Add preference management
    - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5_

  - [ ] 13.2 Implement notification business logic
    - Add notification creation for events
    - Implement email sending with lettre
    - Add preference checking
    - _Requirements: 8.1, 8.2, 8.3, 8.4_

  - [ ]* 13.3 Write property test for event-triggered notifications
    - **Property 29: Event-Triggered Notifications**
    - **Validates: Requirements 8.1, 8.2, 8.3**

  - [ ]* 13.4 Write property test for notification preferences
    - **Property 30: Notification Preference Enforcement**
    - **Validates: Requirements 8.4**

  - [ ]* 13.5 Write property test for preference persistence
    - **Property 31: Notification Preference Persistence**
    - **Validates: Requirements 8.5**

- [ ] 14. Checkpoint - Ensure notification tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 15. Implement security module
  - [ ] 15.1 Create encryption service
    - Implement AES encryption and decryption
    - Add key management
    - _Requirements: 9.1_

  - [ ] 15.2 Implement access control middleware
    - Add role-based access control
    - Implement permission checking
    - _Requirements: 9.2_

  - [ ] 15.3 Implement data export and deletion
    - Add member data export functionality
    - Implement data deletion with transaction preservation
    - _Requirements: 9.3, 9.4_

  - [ ] 15.4 Implement audit logging
    - Add audit log creation for sensitive data access
    - _Requirements: 9.5_

  - [ ]* 15.5 Write property test for sensitive data encryption
    - **Property 32: Sensitive Data Encryption**
    - **Validates: Requirements 9.1**

  - [ ]* 15.6 Write property test for access control
    - **Property 33: Access Control Enforcement**
    - **Validates: Requirements 9.2**

  - [ ]* 15.7 Write property test for data export completeness
    - **Property 34: Data Export Completeness**
    - **Validates: Requirements 9.3**

  - [ ]* 15.8 Write property test for data deletion
    - **Property 35: Data Deletion Preserves Transactions**
    - **Validates: Requirements 9.4**

  - [ ]* 15.9 Write property test for audit logging
    - **Property 36: Audit Logging Invariant**
    - **Validates: Requirements 9.5**

- [ ] 16. Implement reporting module
  - [ ] 16.1 Create repository layer for reports
    - Implement queries for sales and purchase reports
    - Add date range filtering
    - Add metrics calculation
    - _Requirements: 10.1, 10.2, 10.3_

  - [ ] 16.2 Implement report generation logic
    - Add report data aggregation
    - Implement CSV export
    - Implement PDF export
    - _Requirements: 10.1, 10.2, 10.4_

  - [ ] 16.3 Implement administrative analytics
    - Add cooperative-wide analytics queries
    - Implement access control for admin features
    - _Requirements: 10.5_

  - [ ]* 16.4 Write property test for report data accuracy
    - **Property 37: Report Data Accuracy**
    - **Validates: Requirements 10.1, 10.2**

  - [ ]* 16.5 Write property test for metrics calculation
    - **Property 38: Report Metrics Calculation**
    - **Validates: Requirements 10.3**

  - [ ]* 16.6 Write property test for report export formats
    - **Property 39: Report Export Format**
    - **Validates: Requirements 10.4**

  - [ ]* 16.7 Write property test for administrative access
    - **Property 40: Administrative Analytics Access**
    - **Validates: Requirements 10.5**

- [ ] 17. Checkpoint - Ensure security and reporting tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 18. Implement HTTP API layer with Axum
  - [ ] 18.1 Create Axum router and middleware setup
    - Set up route handlers
    - Add authentication middleware
    - Add error handling middleware
    - Add logging middleware
    - _Requirements: All (API layer)_

  - [ ] 18.2 Implement authentication endpoints
    - POST /api/auth/register
    - POST /api/auth/login
    - _Requirements: 1.1, 1.2, 1.3, 1.4_

  - [ ] 18.3 Implement product listing endpoints
    - POST /api/listings
    - GET /api/listings/:id
    - PUT /api/listings/:id
    - DELETE /api/listings/:id
    - GET /api/listings (with search and filters)
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 7.1, 7.2, 7.3, 7.4_

  - [ ] 18.4 Implement order endpoints
    - POST /api/orders
    - GET /api/orders/:id
    - POST /api/orders/:id/accept
    - POST /api/orders/:id/reject
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

  - [ ] 18.5 Implement transaction endpoints
    - GET /api/transactions/:id
    - POST /api/transactions/:id/confirm
    - _Requirements: 4.1, 4.2, 4.3_

  - [ ] 18.6 Implement governance endpoints
    - POST /api/proposals
    - GET /api/proposals
    - POST /api/proposals/:id/vote
    - POST /api/proposals/:id/finalize
    - _Requirements: 5.1, 5.2, 5.3_

  - [ ] 18.7 Implement reputation endpoints
    - POST /api/ratings
    - GET /api/members/:id/reputation
    - _Requirements: 6.1, 6.2, 6.3_

  - [ ] 18.8 Implement notification endpoints
    - GET /api/notifications
    - PUT /api/notifications/preferences
    - _Requirements: 8.5_

  - [ ] 18.9 Implement security endpoints
    - GET /api/members/me/data
    - DELETE /api/members/me/data
    - _Requirements: 9.3, 9.4_

  - [ ] 18.10 Implement reporting endpoints
    - GET /api/reports/sales
    - GET /api/reports/purchases
    - GET /api/reports/analytics (admin only)
    - _Requirements: 10.1, 10.2, 10.5_

- [ ]* 19. Write integration tests for API endpoints
  - Test complete workflows: registration → listing → order → transaction
  - Test authentication and authorization
  - Test error responses
  - _Requirements: All_

- [ ] 20. Final checkpoint - Complete system validation
  - Run all tests (unit, property, integration)
  - Verify all 40 correctness properties pass
  - Ensure all requirements are covered
  - Ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties
- Unit tests validate specific examples and edge cases
- Integration tests validate end-to-end workflows
- All tests use proptest with minimum 100 iterations for property tests
