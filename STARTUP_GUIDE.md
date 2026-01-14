# DOFTA Full Stack dApp - Complete Setup Guide

## 🎯 Overview

DOFTA is a complete Web3 decentralized farmers marketplace with:
- **Frontend**: React + TypeScript + NEAR wallet integration
- **Backend**: Rust/Axum REST API + PostgreSQL
- **Smart Contracts**: NEAR Protocol (Rust) for escrow

## 🚀 Quick Start (Local Development)

### Prerequisites

- **Node.js** 18+ (for frontend)
- **Rust** 1.70+ (for backend & contracts)
- **PostgreSQL** 14+ (for backend database)
- **NEAR CLI** (for contract deployment)

### 1. Backend Setup

```bash
# Navigate to backend
cd backend

# Create database
createdb dofta

# Copy environment file
cp ../.env.example .env

# Edit .env with your database credentials
# DATABASE_URL=postgresql://postgres:password@localhost/dofta
# JWT_SECRET=your-secret-key-change-in-production

# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run

# Start backend server
cargo run
```

Backend will run on `http://localhost:8080`

### 2. Frontend Setup

```bash
# Navigate to frontend
cd frontend

# Install dependencies
npm install

# Copy environment file
cp .env.example .env

# Edit .env
# VITE_API_URL=http://localhost:8080/api
# VITE_NEAR_CONTRACT_ID=your-contract.testnet
# VITE_NEAR_NETWORK=testnet

# Start development server
npm run dev
```

Frontend will run on `http://localhost:3000`

### 3. Smart Contract Setup (Optional for local testing)

```bash
# Navigate to contracts
cd contracts

# Build contract (Windows)
.\build.ps1

# OR (Linux/Mac)
./build.sh

# Deploy to NEAR testnet (requires NEAR account)
near login
near deploy --accountId YOUR_ACCOUNT.testnet --wasmFile res/dofta_marketplace.wasm

# Initialize contract
near call YOUR_ACCOUNT.testnet new '{"owner": "YOUR_ACCOUNT.testnet", "platform_fee_percentage": 2}' --accountId YOUR_ACCOUNT.testnet
```

## 📋 Complete Workflow

### 1. Start All Services

**Terminal 1 - Backend**:
```bash
cd backend
cargo run
```

**Terminal 2 - Frontend**:
```bash
cd frontend
npm run dev
```

### 2. Test the Application

1. **Open browser**: `http://localhost:3000`
2. **Connect NEAR wallet**: Click "Connect Wallet" button
3. **Register account**: Use backend API or frontend form
4. **Create listing**: Add a product for sale
5. **Place order**: Buy a product (creates escrow on NEAR)
6. **Complete order**: Confirm delivery (releases funds)

## 🔧 Environment Configuration

### Backend (.env)
```env
DATABASE_URL=postgresql://postgres:password@localhost/dofta
JWT_SECRET=your-super-secret-jwt-key-min-32-chars
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

### Frontend (.env)
```env
VITE_API_URL=http://localhost:8080/api
VITE_NEAR_CONTRACT_ID=dofta-marketplace.testnet
VITE_NEAR_NETWORK=testnet
```

## 📡 API Endpoints

### Authentication
- `POST /api/auth/register` - Register new member
- `POST /api/auth/login` - Login
- `GET /api/auth/profile` - Get profile (auth required)

### Listings
- `GET /api/listings` - Get all listings
- `POST /api/listings` - Create listing (auth required)
- `GET /api/listings/:id` - Get listing
- `PUT /api/listings/:id` - Update listing (auth required)
- `DELETE /api/listings/:id` - Delete listing (auth required)

### Orders
- `GET /api/orders` - Get my orders (auth required)
- `POST /api/orders` - Create order (auth required)
- `GET /api/orders/:id` - Get order (auth required)
- `PUT /api/orders/:id/status` - Update status (auth required)

## 🧪 Testing

### Backend Tests
```bash
cd backend
cargo test
```

### Frontend Tests
```bash
cd frontend
npm run lint
```

### Contract Tests
```bash
cd contracts
cargo test
```

## 🐛 Troubleshooting

### Backend won't start
- Check PostgreSQL is running: `pg_isready`
- Verify database exists: `psql -l | grep dofta`
- Check .env file has correct DATABASE_URL

### Frontend can't connect to backend
- Verify backend is running on port 8080
- Check CORS is enabled (it is by default)
- Verify VITE_API_URL in frontend/.env

### NEAR wallet won't connect
- Make sure you have a NEAR testnet account
- Check VITE_NEAR_NETWORK is set to "testnet"
- Try clearing browser cache

## 📦 Building for Production

### Backend
```bash
cd backend
cargo build --release
./target/release/dofta
```

### Frontend
```bash
cd frontend
npm run build
# Serve the dist/ folder with any static server
```

### Contracts
```bash
cd contracts
./build.sh  # or build.ps1 on Windows
# Deploy to mainnet with your production account
```

## 🎯 Next Steps

1. **Deploy backend** to a cloud provider (AWS, DigitalOcean, etc.)
2. **Deploy frontend** to Vercel, Netlify, or similar
3. **Deploy contract** to NEAR mainnet
4. **Set up domain** and SSL certificates
5. **Configure production** environment variables

## 📚 Documentation

- [Backend README](./backend/README.md)
- [Frontend README](./frontend/README.md)
- [Contracts README](./contracts/README.md)

## 🤝 Contributing

Contributions welcome! Please test thoroughly before submitting PRs.

## 📄 License

MIT License - see [LICENSE](./LICENSE)

---

**Built with ❤️ for organic farmers worldwide** 🌾
