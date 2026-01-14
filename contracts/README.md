# DOFTA NEAR Smart Contracts

Rust smart contracts for the DOFTA decentralized farmers marketplace, built on NEAR Protocol.

## 🌟 Features

- **Escrow System**: Secure payment locking until order completion
- **Order Management**: Create, complete, refund, and dispute orders
- **Platform Fees**: Configurable platform fee percentage
- **Buyer Protection**: Refund mechanism for disputes
- **Seller Protection**: Funds released only on buyer confirmation
- **Transparent**: All transactions recorded on NEAR blockchain

## 🏗️ Contract Structure

### Main Functions

#### `new(owner, platform_fee_percentage)`
Initialize the contract with an owner and platform fee (0-10%)

#### `create_order(order_id, seller, listing_id, quantity)` [payable]
Create an escrow order by depositing NEAR tokens
- Buyer attaches payment
- Funds locked in escrow
- Returns order details

#### `complete_order(order_id)`
Complete order and release funds to seller (buyer only)
- Deducts platform fee
- Transfers remaining amount to seller
- Marks order as completed

#### `refund_order(order_id)`
Refund order to buyer (seller or owner only)
- Returns full amount to buyer
- Marks order as refunded

#### `dispute_order(order_id)`
Raise a dispute (buyer or seller)
- Marks order as disputed
- Requires owner intervention

#### View Functions
- `get_order(order_id)` - Get order details
- `get_buyer_orders(buyer)` - Get all buyer orders
- `get_seller_orders(seller)` - Get all seller orders
- `get_platform_fee()` - Get current platform fee

## 🚀 Setup

### Prerequisites
- Rust 1.70+
- NEAR CLI
- WASM target

### Installation

1. **Install Rust** (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Install NEAR CLI**:
```bash
npm install -g near-cli
```

3. **Add WASM target**:
```bash
rustup target add wasm32-unknown-unknown
```

4. **Create NEAR testnet account**:
```bash
near login
```

## 🔨 Build

### Linux/Mac:
```bash
./build.sh
```

### Windows:
```powershell
.\build.ps1
```

### Manual build:
```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM file will be in `res/dofta_marketplace.wasm`

## 🧪 Testing

Run unit tests:
```bash
cargo test
```

Run integration tests:
```bash
cargo test --test integration_tests
```

## 📦 Deployment

### Deploy to Testnet

1. **Deploy contract**:
```bash
near deploy --accountId YOUR_ACCOUNT.testnet --wasmFile res/dofta_marketplace.wasm
```

2. **Initialize contract**:
```bash
near call YOUR_ACCOUNT.testnet new '{"owner": "YOUR_ACCOUNT.testnet", "platform_fee_percentage": 2}' --accountId YOUR_ACCOUNT.testnet
```

### Deploy to Mainnet

1. **Create mainnet account** (if needed)
2. **Deploy**:
```bash
near deploy --accountId YOUR_ACCOUNT.near --wasmFile res/dofta_marketplace.wasm
```

3. **Initialize**:
```bash
near call YOUR_ACCOUNT.near new '{"owner": "YOUR_ACCOUNT.near", "platform_fee_percentage": 2}' --accountId YOUR_ACCOUNT.near
```

## 💡 Usage Examples

### Create an Order
```bash
near call CONTRACT_ID create_order '{
  "order_id": "order_123",
  "seller": "farmer.testnet",
  "listing_id": "listing_456",
  "quantity": 10
}' --accountId buyer.testnet --amount 5
```
*Attaches 5 NEAR as payment*

### Complete Order (Buyer)
```bash
near call CONTRACT_ID complete_order '{
  "order_id": "order_123"
}' --accountId buyer.testnet
```

### Refund Order (Seller)
```bash
near call CONTRACT_ID refund_order '{
  "order_id": "order_123"
}' --accountId farmer.testnet
```

### Get Order Details
```bash
near view CONTRACT_ID get_order '{"order_id": "order_123"}'
```

### Get Buyer Orders
```bash
near view CONTRACT_ID get_buyer_orders '{"buyer": "buyer.testnet"}'
```

## 🔐 Security Features

- ✅ **Input Validation**: All inputs validated
- ✅ **Access Control**: Role-based permissions
- ✅ **Reentrancy Protection**: NEAR SDK built-in protection
- ✅ **Overflow Protection**: Rust's safe arithmetic
- ✅ **State Consistency**: Atomic state updates

## 🏛️ Contract Architecture

```
MarketplaceContract
├── owner: AccountId
├── orders: UnorderedMap<String, EscrowOrder>
└── platform_fee_percentage: u8

EscrowOrder
├── order_id: String
├── buyer: AccountId
├── seller: AccountId
├── amount: Balance
├── listing_id: String
├── quantity: u32
├── status: OrderStatus
├── created_at: u64
└── completed_at: Option<u64>

OrderStatus
├── Pending
├── Completed
├── Refunded
└── Disputed
```

## 🔄 Integration with Frontend

The frontend uses `near-api-js` to interact with this contract:

```typescript
import { connect, Contract } from 'near-api-js';

// Create order
await contract.create_order({
  order_id: 'order_123',
  seller: 'farmer.testnet',
  listing_id: 'listing_456',
  quantity: 10
}, '300000000000000', '5000000000000000000000000'); // gas, attached NEAR
```

See `frontend/src/services/near.ts` for full integration.

## 📊 Gas Costs

Approximate gas costs on NEAR testnet:
- `create_order`: ~5 TGas
- `complete_order`: ~10 TGas (includes transfers)
- `refund_order`: ~10 TGas
- `dispute_order`: ~3 TGas
- View calls: Free

## 🛠️ Development

### Project Structure
```
contracts/
├── src/
│   └── lib.rs          # Main contract code
├── tests/
│   └── integration.rs  # Integration tests
├── Cargo.toml          # Dependencies
├── build.sh            # Build script (Linux/Mac)
├── build.ps1           # Build script (Windows)
└── README.md
```

### Adding New Features

1. Edit `src/lib.rs`
2. Add tests
3. Build and test locally
4. Deploy to testnet
5. Test on testnet
6. Deploy to mainnet

## 📝 License

MIT License - see [LICENSE](../LICENSE)

## 🤝 Contributing

Contributions welcome! Please test thoroughly before submitting PRs.

---

**Built with 🦀 Rust on NEAR Protocol**
