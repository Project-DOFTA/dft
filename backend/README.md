# DOFTA Backend API

Rust backend API for the DOFTA farmers cooperative marketplace.

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+
- PostgreSQL 14+
- SQLx CLI

### Setup

1. **Install SQLx CLI**:
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

2. **Create database**:
```bash
createdb dofta
```

3. **Copy environment file**:
```bash
cp ../.env.example .env
```

4. **Configure `.env`**:
```env
DATABASE_URL=postgresql://postgres:password@localhost/dofta
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

5. **Run migrations**:
```bash
sqlx migrate run
```

6. **Build and run**:
```bash
cargo run
```

The API will be available at `http://localhost:8080`

## 📡 API Endpoints

### Health Check
```
GET /health
```

### Authentication
```
POST /api/auth/register - Register new member
POST /api/auth/login - Login existing member
GET /api/auth/profile - Get current user profile (requires auth)
```

### Listings
```
GET /api/listings - Get all listings (with optional filters)
POST /api/listings - Create new listing (requires auth)
GET /api/listings/:id - Get listing by ID
PUT /api/listings/:id - Update listing (requires auth, owner only)
DELETE /api/listings/:id - Delete listing (requires auth, owner only)
```

### Orders
```
GET /api/orders - Get my orders (requires auth)
POST /api/orders - Create new order (requires auth)
GET /api/orders/:id - Get order by ID (requires auth)
PUT /api/orders/:id/status - Update order status (requires auth)
```

## 🔐 Authentication

The API uses JWT (JSON Web Tokens) for authentication.

### Register
```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "farmer@example.com",
    "password": "securepassword",
    "name": "John Farmer",
    "farm_name": "Green Valley Farm",
    "location": "California"
  }'
```

Response:
```json
{
  "member": {
    "id": "uuid",
    "email": "farmer@example.com",
    "name": "John Farmer",
    ...
  },
  "token": "eyJ0eXAiOiJKV1QiLCJhbGc..."
}
```

### Login
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "farmer@example.com",
    "password": "securepassword"
  }'
```

### Using the Token
```bash
curl -X GET http://localhost:8080/api/auth/profile \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## 📦 Project Structure

```
backend/
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library root
│   ├── config.rs         # Configuration
│   ├── db.rs             # Database connection
│   ├── error.rs          # Error types
│   ├── models.rs         # Data models
│   ├── auth.rs           # Authentication logic
│   ├── listings.rs       # Listing business logic
│   ├── orders.rs         # Order business logic
│   ├── handlers/         # HTTP request handlers
│   │   ├── auth.rs
│   │   ├── listings.rs
│   │   └── orders.rs
│   ├── middleware/       # HTTP middleware
│   │   └── auth.rs
│   └── routes.rs         # Route configuration
├── migrations/           # Database migrations
└── Cargo.toml
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Run specific test
cargo test test_name
```

## 🛠️ Development

### Add a new migration
```bash
sqlx migrate add migration_name
```

### Check code without building
```bash
cargo check
```

### Format code
```bash
cargo fmt
```

### Lint code
```bash
cargo clippy
```

## 🔧 Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `JWT_SECRET` | Secret key for JWT tokens | Required |
| `SERVER_HOST` | Server bind address | `127.0.0.1` |
| `SERVER_PORT` | Server port | `8080` |

## 📝 License

MIT
