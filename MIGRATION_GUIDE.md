# Migration Guide: Python SDK to Rust SDK

This guide helps you migrate from the Paradex Python SDK to the Rust SDK.

## Installation

### Python
```bash
pip install paradex-py
```

### Rust
```toml
[dependencies]
paradex-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Basic Client Initialization

### Python
```python
from paradex_py import Paradex
from paradex_py.environment import Environment

paradex = Paradex(env=Environment.TESTNET)
```

### Rust
```rust
use paradex_rs::{Paradex, Environment};

let paradex = Paradex::new(Environment::Testnet)?;
```

## With Authentication

### Python
```python
paradex = Paradex(
    env=Environment.TESTNET,
    l1_address="0x...",
    l1_private_key="0x..."
)
```

### Rust (TODO: Not yet implemented)
```rust
let paradex = Paradex::with_l1_credentials(
    Environment::Testnet,
    "0x...",
    "0x..."
)?;
```

## Fetching System Config

### Python
```python
config = paradex.api_client.fetch_system_config()
```

### Rust
```rust
let config = paradex.api_client().fetch_system_config().await?;
```

## Fetching Markets

### Python
```python
markets = paradex.api_client.fetch_markets()
```

### Rust
```rust
let markets = paradex.api_client().fetch_markets().await?;
```

## Creating an Order

### Python
```python
from paradex_py.common.order import Order
from paradex_py.common.order import OrderSide, OrderType

order = Order(
    market="BTC-USD-PERP",
    order_side=OrderSide.BUY,
    order_type=OrderType.LIMIT,
    size="0.1",
    price="50000"
)
```

### Rust
```rust
use paradex_rs::{Order, OrderSide, OrderType};

let order = Order::builder()
    .market("BTC-USD-PERP")
    .side(OrderSide::Buy)
    .order_type(OrderType::Limit)
    .size("0.1")
    .price("50000")
    .build()?;
```

## Submitting an Order

### Python
```python
result = paradex.api_client.submit_order(order)
```

### Rust
```rust
let result = paradex.api_client().submit_order(&order).await?;
```

## WebSocket Subscriptions

### Python
```python
from paradex_py.api.ws_client import ParadexWebsocketChannel

async def on_message(channel, message):
    print(f"Received: {message}")

await paradex.ws_client.connect()
await paradex.ws_client.subscribe(
    ParadexWebsocketChannel.MARKETS_SUMMARY,
    callback=on_message,
    params={"market": "BTC-USD-PERP"}
)
```

### Rust (TODO: Not yet fully implemented)
```rust
use paradex_rs::WebSocketChannel;

// TODO: WebSocket implementation
```

## Error Handling

### Python
```python
try:
    result = paradex.api_client.fetch_markets()
except Exception as e:
    print(f"Error: {e}")
```

### Rust
```rust
match paradex.api_client().fetch_markets().await {
    Ok(markets) => println!("Markets: {:?}", markets),
    Err(e) => eprintln!("Error: {}", e),
}

// Or using ?
let markets = paradex.api_client().fetch_markets().await?;
```

## Key Differences

### Async/Await
- **Python**: Uses `asyncio` and `await` keywords
- **Rust**: Uses `tokio` runtime and `.await` syntax

### Error Handling
- **Python**: Uses exceptions (`try/except`)
- **Rust**: Uses `Result<T, E>` type and `?` operator

### Type System
- **Python**: Dynamic typing with optional type hints
- **Rust**: Static typing with compile-time guarantees

### Naming Conventions
- **Python**: snake_case for everything
- **Rust**: snake_case for functions/variables, PascalCase for types/enums

## Feature Availability

| Feature | Python SDK | Rust SDK |
|---------|-----------|----------|
| REST API (public) | ✅ | ✅ |
| REST API (private) | ✅ | ✅ |
| Account management | ✅ | 🚧 In progress |
| Order signing | ✅ | 🚧 In progress |
| WebSocket client | ✅ | 🚧 In progress |
| Block trades | ✅ | ⏳ Planned |

Legend:
- ✅ Available
- 🚧 In progress
- ⏳ Planned
- ❌ Not available

## Next Steps

1. Review the [README](./README.md) for complete feature status
2. Check the [examples](./examples) directory for code samples
3. Contribute to missing features on [GitHub](#)

## Getting Help

- Python SDK Docs: https://docs.paradex.trade
- Rust SDK Issues: [GitHub Issues](#)
- Discord: [Paradex Discord](#)
