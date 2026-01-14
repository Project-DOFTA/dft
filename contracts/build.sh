#!/bin/bash
set -e

echo "🔨 Building NEAR smart contract..."

# Add WASM target if not already added
rustup target add wasm32-unknown-unknown

# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Copy the built WASM file to a more accessible location
mkdir -p res
cp target/wasm32-unknown-unknown/release/dofta_marketplace.wasm res/

echo "✅ Contract built successfully!"
echo "📦 WASM file: res/dofta_marketplace.wasm"
echo ""
echo "Next steps:"
echo "1. Deploy to testnet: near deploy --accountId YOUR_ACCOUNT.testnet --wasmFile res/dofta_marketplace.wasm"
echo "2. Initialize: near call YOUR_ACCOUNT.testnet new '{\"owner\": \"YOUR_ACCOUNT.testnet\", \"platform_fee_percentage\": 2}' --accountId YOUR_ACCOUNT.testnet"
