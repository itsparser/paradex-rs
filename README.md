# Paradex Rust SDK

**High-performance Rust implementation of the Paradex SDK for decentralized derivatives trading**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> Complete feature parity with the [Paradex Python SDK](../paradex-py), built with Rust's performance and type safety.

---

## ⚡ Quick Start

```rust
use paradex_rs::{Paradex, Environment, Order, OrderSide, OrderType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let paradex = Paradex::with_l1_credentials(
        Environment::Testnet,
        "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
        "0x1234..."
    ).await?;

    // Fetch markets
    let markets = paradex.api_client().lock().unwrap()
        .fetch_markets().await?;

    // Create and submit order
    let mut order = Order::builder()
        .market("BTC-USD-PERP")
        .side(OrderSide::Buy)
        .order_type(OrderType::Limit)
        .size("0.1")
        .price("50000")
        .build()?;

    paradex.account().unwrap().lock().unwrap().sign_order(&mut order)?;
    let result = paradex.api_client().lock().unwrap()
        .submit_order(&order).await?;

    println!("Order submitted: {}", result.id);
    Ok(())
}
```

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
paradex-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

---

## ✨ Features

### ✅ Complete Implementation (100% Feature Parity)

- **Account Management**
  - L1 to L2 key derivation from Ethereum
  - Stark key generation
  - Account address computation
  - Subkey support for L2-only authentication

- **Message Signing**
  - Order signing (EIP-712 on Starknet)
  - Authentication & onboarding signing
  - Block trade & offer signing
  - Fullnode RPC message signing

- **REST API (50+ endpoints)**
  - Public: Markets, orderbook, trades, funding
  - Private: Account, orders, positions, fills
  - Batch operations (submit, cancel)
  - Block trades (10+ endpoints)

- **WebSocket (15+ channels)**
  - Real-time market data (BBO, trades, orderbook)
  - Account updates (orders, fills, positions)
  - Auto-reconnect & ping/pong
  - Type-safe callback system

- **Authentication**
  - JWT token management
  - Auto-refresh (4-minute interval)
  - Onboarding flow

---

## 📚 Usage Examples

### Public Market Data

```rust
use paradex_rs::{Paradex, Environment};

let paradex = Paradex::new(Environment::Testnet)?;

// Fetch markets
let markets = paradex.api_client().lock().unwrap()
    .fetch_markets().await?;

// Get orderbook
let orderbook = paradex.api_client().lock().unwrap()
    .fetch_orderbook("BTC-USD-PERP", Some(10)).await?;

// Get BBO (best bid/offer)
let bbo = paradex.api_client().lock().unwrap()
    .fetch_bbo("BTC-USD-PERP").await?;
```

### Authenticated Trading

```rust
// Initialize with L1 credentials (derives L2 key)
let paradex = Paradex::with_l1_credentials(
    Environment::Testnet,
    "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "0x1234567890abcdef..."
).await?;

// Or use L2 key directly
let paradex = Paradex::with_l2_credentials(
    Environment::Testnet,
    "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "0xabcdef1234567890..."
).await?;

// Fetch account summary
let summary = paradex.api_client().lock().unwrap()
    .fetch_account_summary().await?;
```

### Order Management

```rust
use paradex_rs::{Order, OrderSide, OrderType, OrderInstruction};

// Create order
let mut order = Order::builder()
    .market("BTC-USD-PERP")
    .side(OrderSide::Buy)
    .order_type(OrderType::Limit)
    .size("0.1")
    .price("50000")
    .instruction(OrderInstruction::PostOnly)
    .client_id("my-order-123")
    .build()?;

// Sign order
paradex.account().unwrap().lock().unwrap()
    .sign_order(&mut order)?;

// Submit order
let result = paradex.api_client().lock().unwrap()
    .submit_order(&order).await?;

// Cancel order
paradex.api_client().lock().unwrap()
    .cancel_order(&result.id).await?;
```

### Batch Operations

```rust
// Submit multiple orders at once
let orders = vec![order1, order2, order3];
let result = paradex.api_client().lock().unwrap()
    .submit_orders_batch(&orders).await?;

// Cancel multiple orders
paradex.api_client().lock().unwrap()
    .cancel_orders_batch(Some(&order_ids), None).await?;
```

### WebSocket Real-time Data

```rust
use paradex_rs::WebSocketChannel;

let ws = paradex.ws_client();
ws.lock().unwrap().connect().await?;

// Subscribe to BBO
ws.lock().unwrap().subscribe(
    WebSocketChannel::BBO,
    Some("BTC-USD-PERP"),
    |message| Box::pin(async move {
        println!("BBO Update: {:?}", message);
    })
).await?;

// Subscribe to trades
ws.lock().unwrap().subscribe(
    WebSocketChannel::Trades,
    Some("BTC-USD-PERP"),
    |message| Box::pin(async move {
        println!("Trade: {:?}", message);
    })
).await?;

// Subscribe to private order updates
ws.lock().unwrap().subscribe(
    WebSocketChannel::Orders,
    Some("BTC-USD-PERP"),
    |message| Box::pin(async move {
        println!("Order Update: {:?}", message);
    })
).await?;
```

### Subkey Authentication (L2-only)

```rust
use paradex_rs::ParadexSubkey;

// No L1 credentials needed
let paradex = ParadexSubkey::new(
    Environment::Testnet,
    "0xabcdef...",  // L2 private key
    "0x123456..."   // L2 address
).await?;

// Full API access with subkey
let positions = paradex.api_client().lock().unwrap()
    .fetch_positions().await?;
```

### Block Trades

```rust
use paradex_rs::{BlockTradeRequest, BlockExecuteRequest};

// Create block trade
let block_trade = BlockTradeRequest {
    markets: vec!["BTC-USD-PERP".to_string()],
    required_signers: vec!["0x123...".to_string()],
    signature: "...".to_string(),
    signature_timestamp: 1234567890,
};

let result = paradex.api_client().lock().unwrap()
    .create_block_trade(&block_trade).await?;

// Execute block trade
let execution = BlockExecuteRequest {
    offer_ids: vec!["offer123".to_string()],
};

paradex.api_client().lock().unwrap()
    .execute_block_trade(&result.block_id, &execution).await?;
```

---

## 🎯 Complete Feature List

### REST API Endpoints (50+)

**System & Markets (Public)**
- `fetch_system_config()` - System configuration
- `fetch_system_state()` - System status
- `fetch_system_time()` - Server time
- `fetch_markets()` - All markets
- `fetch_markets_summary()` - Market summaries
- `fetch_klines()` - OHLCV candlestick data
- `fetch_orderbook()` - Order book
- `fetch_bbo()` - Best bid/offer
- `fetch_trades()` - Public trades
- `fetch_funding_data()` - Funding data
- `fetch_insurance_fund()` - Insurance fund info

**Account (Private)**
- `fetch_account_summary()` - Account summary
- `fetch_account_profile()` - Profile data
- `fetch_account_info()` - Account info
- `fetch_subaccounts()` - Sub-accounts list
- `fetch_balances()` - All balances
- `fetch_positions()` - All positions

**Orders (Private)**
- `fetch_orders()` - Open orders
- `fetch_orders_history()` - Order history
- `fetch_order()` - Get order by ID
- `fetch_order_by_client_id()` - Get order by client ID
- `submit_order()` - Submit single order
- `submit_orders_batch()` - Submit multiple orders
- `modify_order()` - Modify existing order
- `cancel_order()` - Cancel order
- `cancel_order_by_client_id()` - Cancel by client ID
- `cancel_all_orders()` - Cancel all orders
- `cancel_orders_batch()` - Cancel multiple orders

**Trading History (Private)**
- `fetch_fills()` - Fill history
- `fetch_tradebusts()` - Tradebust history
- `fetch_funding_payments()` - Funding payments
- `fetch_transactions()` - Transaction history
- `fetch_transfers()` - Transfer history
- `fetch_liquidations()` - Liquidation history
- `fetch_points_data()` - Points program data

**Block Trades (Private)**
- `list_block_trades()` - List all block trades
- `create_block_trade()` - Create new block trade
- `get_block_trade()` - Get block trade details
- `cancel_block_trade()` - Cancel block trade
- `execute_block_trade()` - Execute block trade
- `get_block_trade_offers()` - Get all offers
- `create_block_trade_offer()` - Create offer
- `get_block_trade_offer()` - Get specific offer
- `cancel_block_trade_offer()` - Cancel offer
- `execute_block_trade_offer()` - Execute offer

### WebSocket Channels (15+)

**Public Channels**
- `BBO` - Best bid/offer updates
- `Trades` - Trade updates
- `OrderBook` - Orderbook snapshots
- `MarketsSummary` - Market summary updates
- `FundingData` - Funding data updates
- `FundingRateComparison` - Funding rate comparison

**Private Channels** (require auth)
- `Account` - Account updates
- `BalanceEvents` - Balance event updates
- `Orders` - Order updates
- `Fills` - Fill updates
- `Positions` - Position updates
- `FundingPayments` - Funding payment updates
- `Tradebusts` - Tradebust notifications
- `Transactions` - Transaction updates
- `Transfers` - Transfer updates

---

## 🏗️ Architecture

```
paradex-rs/
├── src/
│   ├── lib.rs                    # Main entry point
│   ├── environment.rs            # Environment config
│   ├── constants.rs              # Constants
│   ├── error.rs                  # Error types
│   ├── subkey.rs                 # Subkey support
│   ├── types/                    # Type definitions
│   │   ├── models.rs             # API models
│   │   ├── order.rs              # Order types
│   │   └── block_trades.rs       # Block trade types
│   ├── account/                  # Account management
│   │   ├── account.rs            # Main account
│   │   ├── key_derivation.rs    # Key derivation
│   │   ├── signing.rs            # Message signing
│   │   └── block_trades_signing.rs  # Block trade signing
│   ├── message/                  # Message builders
│   │   ├── typed_data.rs         # EIP-712 structures
│   │   ├── order.rs              # Order messages
│   │   ├── auth.rs               # Auth messages
│   │   ├── onboarding.rs         # Onboarding messages
│   │   └── block_trades.rs       # Block trade messages
│   ├── api/                      # API clients
│   │   ├── client.rs             # REST API (50+ endpoints)
│   │   ├── http_client.rs        # HTTP wrapper
│   │   ├── auth.rs               # Authentication
│   │   ├── block_trades.rs       # Block trades API
│   │   ├── ws_client.rs          # WebSocket facade
│   │   └── ws_client_impl.rs     # WebSocket implementation
│   └── utils/                    # Utilities
│       └── mod.rs                # Helper functions
└── examples/                     # Examples
    ├── basic_api_usage.rs        # Public API
    ├── order_management.rs       # Orders
    ├── fetch_account_data.rs     # Account data
    ├── complete_workflow.rs      # Full workflow
    └── websocket_realtime.rs     # WebSocket
```

---

## 🚀 Examples

```bash
# Public API (no auth required)
cargo run --example basic_api_usage

# Order management (requires credentials)
cargo run --example order_management

# Complete workflow
cargo run --example complete_workflow

# WebSocket real-time data
cargo run --example websocket_realtime
```

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test -- --nocapture

# Build documentation
cargo doc --open
```

---

## 📊 Feature Comparison

| Feature | Python SDK | Rust SDK |
|---------|:----------:|:--------:|
| **Core** |
| Environment config | ✅ | ✅ |
| Error handling | ✅ | ✅ |
| **Account** |
| L1 key derivation | ✅ | ✅ |
| L2 key management | ✅ | ✅ |
| Subkey support | ✅ | ✅ |
| Message signing | ✅ | ✅ |
| **REST API** |
| 50+ endpoints | ✅ | ✅ |
| Batch operations | ✅ | ✅ |
| Block trades (10 endpoints) | ✅ | ✅ |
| **WebSocket** |
| 15+ channels | ✅ | ✅ |
| Auto-reconnect | ✅ | ✅ |
| Callbacks | ✅ | ✅ |
| **Auth** |
| JWT management | ✅ | ✅ |
| Auto-refresh | ✅ | ✅ |
| **Overall** | **100%** | **100%** |

---

## 🎓 Documentation

- **[Migration Guide](MIGRATION_GUIDE.md)** - Python → Rust transition
- **[API Documentation](https://docs.rs/paradex-rs)** - Full API docs
- **[Examples](examples/)** - Working code samples
- **[Paradex Docs](https://docs.paradex.trade)** - Official API docs

---

## ⚙️ Advanced Features

### Custom Configuration

```rust
// With custom timeouts and settings
let paradex = Paradex::new(Environment::Testnet)?;

// Manual auth refresh
paradex.refresh_auth_if_needed().await?;
```

### Type-Safe Operations

```rust
// Compile-time type checking
let order = Order::builder()
    .market("BTC-USD-PERP")
    .side(OrderSide::Buy)  // Enum, not string
    .order_type(OrderType::Limit)
    .size("0.1")
    .build()?;  // Returns Result
```

### Error Handling

```rust
use paradex_rs::ParadexError;

match paradex.api_client().lock().unwrap().fetch_markets().await {
    Ok(markets) => println!("Got {} markets", markets.results.len()),
    Err(ParadexError::ApiError { status, message }) => {
        eprintln!("API error {}: {}", status, message);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## 🔑 Authentication Methods

### 1. L1 Credentials (Recommended)
```rust
let paradex = Paradex::with_l1_credentials(
    Environment::Testnet,
    "0xYourEthAddress",
    "0xYourEthPrivateKey"
).await?;
```

### 2. L2 Credentials
```rust
let paradex = Paradex::with_l2_credentials(
    Environment::Testnet,
    "0xYourEthAddress",
    "0xYourL2PrivateKey"
).await?;
```

### 3. Subkey (L2-only)
```rust
let paradex = ParadexSubkey::new(
    Environment::Testnet,
    "0xYourL2PrivateKey",
    "0xYourL2Address"
).await?;
```

---

## 💡 Why Rust SDK?

- **Type Safety** - Compile-time guarantees, no runtime surprises
- **Performance** - Zero-cost abstractions, minimal allocations
- **Memory Safety** - No garbage collector, predictable performance
- **Concurrency** - Safe concurrent operations with Tokio
- **Reliability** - Strong error handling, no exceptions

---

## 🤝 Contributing

Contributions welcome! Areas for enhancement:

- Additional examples and tutorials
- Performance optimizations
- Extended test coverage
- Documentation improvements

---

## 📝 License

MIT License - see [LICENSE](LICENSE) for details

---

## 🔗 Links

- [Paradex Official Site](https://paradex.trade)
- [Paradex Documentation](https://docs.paradex.trade)
- [Python SDK](../paradex-py)
- [API Reference](https://docs.rs/paradex-rs)

---

## ⚠️ Disclaimer

This SDK is provided as-is. Always test thoroughly on testnet before using in production. Not affiliated with or endorsed by Paradex.

---

**Built with ❤️ in Rust**
