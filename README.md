# DOFTA - Decentralized Organic Farmers Trading Alliance

A Web3-powered cooperative platform connecting organic farmers directly with buyers through blockchain technology.

## 🌟 Project Structure

```
dft/
├── frontend/          # React + TypeScript + Solana Web3 UI
├── backend/           # Rust/Axum API + PostgreSQL
├── contracts/         # Solana smart contracts (Anchor)
└── README.md
```

## 🚀 Quick Start

### Frontend (React + Web3)
```bash
cd frontend
npm install
npm run dev
```
Visit `http://localhost:3000`

### Backend (Rust API)
```bash
cd backend
cargo build
cargo run
```
API runs on `http://localhost:8080`

### Smart Contracts (Coming Soon)
```bash
cd contracts
anchor build
anchor test
```

## 📋 Features

### ✅ Completed
- **Backend API**: Authentication, product listings, order management
- **Database**: PostgreSQL with full migrations
- **Frontend**: React + TypeScript with Solana wallet integration
- **UI/UX**: Modern glassmorphism design with TailwindCSS
- **Web3**: Phantom & Solflare wallet support

### 🚧 In Progress
- Frontend marketplace pages
- HTTP API endpoints
- Backend-frontend integration

### 📅 Planned
- Solana smart contracts for escrow
- On-chain transaction recording
- Governance & voting
- Reputation system

## 🛠️ Technology Stack

| Component | Technology |
|-----------|-----------|
| Frontend | React 18, TypeScript, Vite, TailwindCSS |
| Web3 | Solana, @solana/wallet-adapter, @project-serum/anchor |
| Backend | Rust, Axum, SQLx, Tokio |
| Database | PostgreSQL |
| Smart Contracts | Solana, Anchor Framework |
| Auth | JWT, Argon2 |

## 📖 Documentation

- [Frontend README](./frontend/README.md)
- [Backend README](./backend/README.md)
- [Contracts README](./contracts/README.md)
- [Implementation Plan](./IMPLEMENTATION_LOG.md)

## 🎯 MVP Goals

1. **Farmers can list products** with pricing and availability
2. **Buyers can browse and order** products
3. **Secure payments** via Solana escrow (smart contracts)
4. **Transaction history** recorded on blockchain
5. **User authentication** with wallet integration

## 🔧 Development Setup

### Prerequisites
- Node.js 18+
- Rust 1.70+
- PostgreSQL 14+
- Solana CLI (for smart contracts)

### Environment Setup

1. **Frontend**:
```bash
cd frontend
cp .env.example .env
npm install
```

2. **Backend**:
```bash
cd backend
cp ../.env.example .env
cargo build
```

3. **Database**:
```bash
createdb dofta
cd backend && sqlx migrate run
```

## 🌐 Architecture

### Hybrid Web3 Architecture
- **On-chain** (Solana): Escrow, transactions, governance
- **Off-chain** (Backend): User profiles, product metadata, search
- **Frontend**: Web3 wallet integration + traditional API calls

### Why This Approach?
✅ **Cost-effective**: Store large data off-chain  
✅ **Fast**: Complex queries use PostgreSQL  
✅ **Decentralized**: Critical transactions on blockchain  
✅ **User-friendly**: Familiar Web2 UX with Web3 benefits  

## 📄 License

MIT License - see [LICENSE](./LICENSE)

## 🤝 Contributing

This is a cooperative project. Contributions welcome!

---

**Built with ❤️ for organic farmers worldwide** 🌾
