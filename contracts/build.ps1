# Build NEAR smart contract
Write-Host "🔨 Building NEAR smart contract..." -ForegroundColor Cyan

# Add WASM target if not already added
rustup target add wasm32-unknown-unknown

# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Copy the built WASM file to a more accessible location
New-Item -ItemType Directory -Force -Path res | Out-Null
Copy-Item target/wasm32-unknown-unknown/release/dofta_marketplace.wasm res/

Write-Host "✅ Contract built successfully!" -ForegroundColor Green
Write-Host "📦 WASM file: res/dofta_marketplace.wasm" -ForegroundColor Yellow
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "1. Deploy to testnet: near deploy --accountId YOUR_ACCOUNT.testnet --wasmFile res/dofta_marketplace.wasm"
Write-Host "2. Initialize: near call YOUR_ACCOUNT.testnet new '{`"owner`": `"YOUR_ACCOUNT.testnet`", `"platform_fee_percentage`": 2}' --accountId YOUR_ACCOUNT.testnet"
