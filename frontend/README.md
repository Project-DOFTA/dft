# DOFTA Frontend

React + TypeScript frontend for the DOFTA decentralized farmers marketplace.

## Features

- 🌐 **Web3 Integration**: Solana wallet adapter (Phantom, Solflare)
- ⚡ **Modern Stack**: React 18 + TypeScript + Vite
- 🎨 **Beautiful UI**: TailwindCSS with glassmorphism design
- 🔐 **Secure**: JWT authentication + blockchain transactions
- 📱 **Responsive**: Mobile-first design

## Setup

1. Install dependencies:
```bash
npm install
```

2. Copy environment variables:
```bash
cp .env.example .env
```

3. Start development server:
```bash
npm run dev
```

The app will be available at `http://localhost:3000`

## Project Structure

```
src/
├── components/     # Reusable UI components
├── contexts/       # React contexts (Wallet, Auth)
├── hooks/          # Custom React hooks
├── pages/          # Route pages
├── services/       # API and blockchain services
└── utils/          # Helper functions
```

## Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint

## Wallet Setup

To use the marketplace, you'll need a Solana wallet:

1. Install [Phantom](https://phantom.app/) or [Solflare](https://solflare.com/)
2. Switch to Devnet in wallet settings
3. Get free SOL from [Solana Faucet](https://faucet.solana.com/)

## Tech Stack

- **React 18** - UI library
- **TypeScript** - Type safety
- **Vite** - Build tool
- **TailwindCSS** - Styling
- **@solana/web3.js** - Solana SDK
- **@solana/wallet-adapter** - Wallet integration
- **Axios** - HTTP client
- **React Router** - Routing

## Environment Variables

- `VITE_API_URL` - Backend API URL (default: http://localhost:8080/api)
- `VITE_SOLANA_NETWORK` - Solana network (devnet/testnet/mainnet-beta)
