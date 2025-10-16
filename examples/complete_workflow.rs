//! Complete workflow example showing all Paradex SDK features
//!
//! This example demonstrates:
//! - Client initialization with authentication
//! - Fetching system config and market data
//! - Account management
//! - Order creation, signing, and submission
//! - WebSocket subscriptions for real-time data
//! - Position and balance queries

use paradex_rs::{Environment, Paradex};
// Note: WebSocketChannel import needed when uncommenting authenticated features
// use paradex_rs::WebSocketChannel;

#[allow(clippy::await_holding_lock)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("=== Paradex Rust SDK - Complete Workflow Example ===\n");

    // Step 1: Initialize Paradex client (public endpoints only)
    println!("Step 1: Initializing Paradex client...");
    let paradex = Paradex::new(Environment::Testnet)?;
    println!("✓ Client initialized\n");

    // Step 2: Fetch system configuration
    println!("Step 2: Fetching system configuration...");
    let config = paradex
        .api_client()
        .lock()
        .unwrap()
        .fetch_system_config()
        .await?;
    println!("✓ System config loaded");
    println!("  - L1 Chain ID: {}", config.l1_chain_id);
    println!("  - L2 Chain ID: {}", config.starknet_chain_id);
    println!("  - Paraclear decimals: {}\n", config.paraclear_decimals);

    // Step 3: Fetch markets
    println!("Step 3: Fetching available markets...");
    let markets = paradex.api_client().lock().unwrap().fetch_markets().await?;
    println!("✓ Found {} markets", markets.results.len());
    if let Some(first_market) = markets.results.first() {
        println!("  - Example market: {}", first_market.symbol);
    }
    println!();

    // Step 4: Fetch market summary
    println!("Step 4: Fetching BTC-USD-PERP market summary...");
    let summary = paradex
        .api_client()
        .lock()
        .unwrap()
        .fetch_markets_summary(Some("BTC-USD-PERP"))
        .await?;
    if let Some(btc_summary) = summary.results.first() {
        println!("✓ Market Summary:");
        println!("  - Symbol: {}", btc_summary.symbol);
        if let Some(last_price) = &btc_summary.last_price {
            println!("  - Last Price: {last_price}");
        }
        if let Some(volume) = &btc_summary.volume_24h {
            println!("  - 24h Volume: {volume}");
        }
    }
    println!();

    // Step 5: Fetch orderbook
    println!("Step 5: Fetching orderbook...");
    let orderbook = paradex
        .api_client()
        .lock()
        .unwrap()
        .fetch_orderbook("BTC-USD-PERP", Some(5))
        .await?;
    println!("✓ Orderbook loaded");
    println!("  - Bids: {}", orderbook.bids.len());
    println!("  - Asks: {}", orderbook.asks.len());
    println!();

    // ==================================================================
    // AUTHENTICATED FEATURES (Uncomment and add credentials to use)
    // ==================================================================

    /*
    // Step 6: Initialize with L1 credentials
    println!("Step 6: Authenticating with L1 credentials...");
    let paradex_auth = Paradex::with_l1_credentials(
        Environment::Testnet,
        "YOUR_ETHEREUM_ADDRESS",
        "YOUR_ETHEREUM_PRIVATE_KEY"
    ).await?;
    println!("✓ Authentication successful\n");

    // Step 7: Fetch account summary
    println!("Step 7: Fetching account summary...");
    let account_summary = paradex_auth.api_client().lock().unwrap()
        .fetch_account_summary().await?;
    println!("✓ Account Summary:");
    println!("  - Account: {}", account_summary.account);
    println!("  - Equity (USD): {}", account_summary.equity_usd);
    println!("  - Buying Power (USD): {}", account_summary.buying_power_usd);
    println!();

    // Step 8: Fetch positions
    println!("Step 8: Fetching positions...");
    let positions = paradex_auth.api_client().lock().unwrap()
        .fetch_positions().await?;
    println!("✓ Found {} positions", positions.results.len());
    for position in positions.results {
        println!("  - Market: {}, Size: {}, Side: {}",
            position.market, position.size, position.side);
    }
    println!();

    // Step 9: Create and sign an order
    println!("Step 9: Creating and signing an order...");
    let mut order = Order::builder()
        .market("BTC-USD-PERP")
        .side(OrderSide::Buy)
        .order_type(OrderType::Limit)
        .size("0.001")  // Small size for testing
        .price("45000")  // Limit price
        .instruction(OrderInstruction::PostOnly)
        .client_id("rust-sdk-example-001")
        .build()?;

    // Sign the order
    let account = paradex_auth.account().unwrap();
    account.lock().unwrap().sign_order(&mut order)?;
    println!("✓ Order created and signed");
    println!("  - Market: {}", order.market);
    println!("  - Side: {}", order.order_side);
    println!("  - Size: {}", order.size);
    println!("  - Price: {}", order.price.as_ref().unwrap());
    println!();

    // Step 10: Submit order (COMMENTED OUT - REMOVE TO ACTUALLY TRADE)
    // println!("Step 10: Submitting order...");
    // let order_result = paradex_auth.api_client().lock().unwrap()
    //     .submit_order(&order).await?;
    // println!("✓ Order submitted successfully");
    // println!("  - Order ID: {}", order_result.id);
    // println!();

    // Step 11: Connect WebSocket and subscribe to channels
    println!("Step 11: Connecting to WebSocket...");
    let ws_client = paradex_auth.ws_client();
    ws_client.lock().unwrap().connect().await?;
    println!("✓ WebSocket connected\n");

    println!("Step 12: Subscribing to market data channels...");

    // Subscribe to BBO (Best Bid/Offer)
    ws_client.lock().unwrap().subscribe(
        WebSocketChannel::BBO,
        Some("BTC-USD-PERP"),
        |message| {
            Box::pin(async move {
                if let Some(data) = message.get("data") {
                    println!("[BBO Update] {:?}", data);
                }
            })
        }
    ).await?;
    println!("✓ Subscribed to BBO channel");

    // Subscribe to Trades
    ws_client.lock().unwrap().subscribe(
        WebSocketChannel::Trades,
        Some("BTC-USD-PERP"),
        |message| {
            Box::pin(async move {
                if let Some(data) = message.get("data") {
                    println!("[Trade] {:?}", data);
                }
            })
        }
    ).await?;
    println!("✓ Subscribed to Trades channel");

    // Subscribe to Orders (private channel)
    ws_client.lock().unwrap().subscribe(
        WebSocketChannel::Orders,
        Some("BTC-USD-PERP"),
        |message| {
            Box::pin(async move {
                if let Some(data) = message.get("data") {
                    println!("[Order Update] {:?}", data);
                }
            })
        }
    ).await?;
    println!("✓ Subscribed to Orders channel");

    println!("\nListening for WebSocket messages for 30 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    ws_client.lock().unwrap().close().await?;
    println!("✓ WebSocket closed");
    */

    println!("\n=== Example completed successfully! ===");
    println!("\nNote: Authenticated features are commented out.");
    println!("Uncomment and add your credentials to test full functionality.");

    Ok(())
}
