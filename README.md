# Paradex Rust SDK

Unofficial Rust implementation of the Paradex SDK for interacting with the Paradex decentralized derivatives exchange.

## 🚧 Development Status

This SDK is currently under active development and is being migrated from the official [Paradex Python SDK](../paradex-py). The goal is to provide feature parity with the Python SDK while leveraging Rust's performance and type safety.

### ✅ Implemented Features

- [x] Environment configuration (Production/Testnet)
- [x] Error handling with custom error types
- [x] Type-safe models for all API responses
- [x] Order types and builders
- [x] HTTP client with authentication support
- [x] Complete REST API client with 40+ endpoints:
  - [x] Public market data endpoints
  - [x] Private account endpoints
  - [x] Order management (submit, modify, cancel)
  - [x] Position and balance queries
  - [x] Fills and trade history
  - [x] Funding and liquidation data
- [x] Basic WebSocket client structure
- [x] Comprehensive examples

### 🚧 In Progress / TODO

- [ ] Account management
  - [ ] L1/L2 key derivation from Ethereum private key
  - [ ] Stark key generation
  - [ ] Account address computation
  - [ ] Ledger hardware wallet support
- [ ] Message signing
  - [ ] Order message signing (EIP-712)
  - [ ] Authentication message signing
  - [ ] Onboarding message signing
  - [ ] Block trade message signing
  - [ ] Fullnode RPC message signing
- [ ] WebSocket client (full implementation)
  - [ ] Connection management with auto-reconnect
  - [ ] 15+ real-time channels
  - [ ] Message validation
  - [ ] Callback system
- [ ] Authentication
  - [ ] JWT token management with auto-refresh
  - [ ] Onboarding flow
  - [ ] Custom auth providers
- [ ] Block trades API
- [ ] L2 transfer functionality
- [ ] Comprehensive test suite
- [ ] Documentation with examples

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
paradex-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## 🚀 Quick Start

### Basic Usage - Public Endpoints

```rust
use paradex_rs::{Paradex, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let paradex = Paradex::new(Environment::Testnet)?;

    // Fetch system configuration
    let config = paradex.api_client().fetch_system_config().await?;
    println!("System config: {:?}", config);

    // Fetch market data
    let markets = paradex.api_client().fetch_markets().await?;
    println!("Found {} markets", markets.results.len());

    // Fetch orderbook
    let orderbook = paradex.api_client()
        .fetch_orderbook("BTC-USD-PERP", Some(10))
        .await?;
    println!("Orderbook: {:?}", orderbook);

    Ok(())
}
```

### Order Management (Requires Authentication)

```rust
use paradex_rs::{Paradex, Environment, Order, OrderSide, OrderType, OrderInstruction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize with L1 credentials
    let paradex = Paradex::with_l1_credentials(
        Environment::Testnet,
        "your_ethereum_address",
        "your_ethereum_private_key"
    )?;

    // Create and submit an order
    let order = Order::builder()
        .market("BTC-USD-PERP")
        .side(OrderSide::Buy)
        .order_type(OrderType::Limit)
        .size("0.1")
        .price("50000")
        .instruction(OrderInstruction::PostOnly)
        .build()?;

    let result = paradex.api_client().submit_order(&order).await?;
    println!("Order submitted: {:?}", result);

    // Fetch account positions
    let positions = paradex.api_client().fetch_positions().await?;
    println!("Positions: {:?}", positions);

    Ok(())
}
```

## 📚 Feature Comparison with Python SDK

| Feature | Python SDK | Rust SDK | Notes |
|---------|-----------|----------|-------|
| **Core** |
| Environment config | ✅ | ✅ | |
| HTTP client | ✅ | ✅ | |
| Error handling | ✅ | ✅ | Enhanced with Rust type system |
| **API Client** |
| Public endpoints | ✅ | ✅ | All 15+ public endpoints |
| Private endpoints | ✅ | ✅ | All 40+ endpoints |
| Order management | ✅ | ✅ | Submit, modify, cancel |
| Batch operations | ✅ | ✅ | |
| **Account** |
| L1 key derivation | ✅ | 🚧 | In progress |
| L2 key management | ✅ | 🚧 | In progress |
| Ledger support | ✅ | 🚧 | Planned |
| Message signing | ✅ | 🚧 | In progress |
| **WebSocket** |
| Connection mgmt | ✅ | 🚧 | Basic structure ready |
| Real-time channels | ✅ | 🚧 | 15+ channels planned |
| Auto-reconnect | ✅ | 🚧 | Planned |
| Message validation | ✅ | 🚧 | Planned |
| **Block Trades** |
| Block trades API | ✅ | 🚧 | Planned |
| **Advanced** |
| Custom HTTP client | ✅ | ✅ | Via reqwest |
| Custom WS connector | ✅ | 🚧 | Planned |
| Retry strategies | ✅ | 🚧 | Planned |
| Request hooks | ✅ | 🚧 | Planned |

## 🏗️ Architecture

The SDK is organized into the following modules:

```
src/
├── lib.rs              # Main entry point
├── environment.rs      # Environment configuration
├── constants.rs        # SDK constants
├── error.rs            # Error types
├── types/              # Type definitions
│   ├── models.rs       # API response models
│   └── order.rs        # Order types
├── api/                # API clients
│   ├── client.rs       # REST API client
│   ├── http_client.rs  # HTTP client wrapper
│   └── ws_client.rs    # WebSocket client
├── account/            # Account management
├── message/            # Message signing
└── utils/              # Utility functions
```

## 📖 Examples

See the `examples/` directory for comprehensive examples:

- `basic_api_usage.rs` - Public API endpoints
- `order_management.rs` - Creating and submitting orders
- `fetch_account_data.rs` - Account data queries

Run examples with:

```bash
cargo run --example basic_api_usage
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

## 🤝 Contributing

This SDK is being developed to provide a robust Rust alternative to the Python SDK. Contributions are welcome!

Priority areas for contribution:
1. Account management and key derivation
2. Message signing (EIP-712)
3. WebSocket client implementation
4. Test coverage
5. Documentation and examples

## 📝 License

MIT

## 🔗 Related Projects

- [Paradex Python SDK](../paradex-py) - Official Python SDK
- [Paradex Documentation](https://docs.paradex.trade) - Official API documentation

## ⚠️ Disclaimer

This is an unofficial SDK and is not affiliated with or endorsed by Paradex. Use at your own risk.

## 📊 Progress Tracker

**Overall Completion: ~40%**

- Core infrastructure: 90%
- REST API client: 95%
- WebSocket client: 20%
- Account management: 10%
- Message signing: 5%
- Documentation: 60%
- Examples: 50%
- Tests: 20%

Last updated: 2025-10-16
