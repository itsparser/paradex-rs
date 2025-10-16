//! WebSocket real-time data example
//!
//! Demonstrates subscribing to multiple WebSocket channels and receiving real-time updates

use paradex_rs::{Environment, Paradex, WebSocketChannel};

#[allow(clippy::await_holding_lock)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("=== Paradex WebSocket Real-time Data Example ===\n");

    // Initialize Paradex client
    let paradex = Paradex::new(Environment::Testnet)?;
    println!("✓ Client initialized\n");

    // Get WebSocket client
    let ws = paradex.ws_client();

    // Connect to WebSocket
    println!("Connecting to WebSocket...");
    ws.lock().unwrap().connect().await?;
    println!("✓ Connected\n");

    // Subscribe to BBO (Best Bid/Offer) for BTC
    println!("Subscribing to BBO channel for BTC-USD-PERP...");
    ws.lock()
        .unwrap()
        .subscribe(WebSocketChannel::BBO, Some("BTC-USD-PERP"), |message| {
            Box::pin(async move {
                if let Some(data) = message.get("data") {
                    if let (Some(bid), Some(ask)) = (data.get("bid"), data.get("ask")) {
                        println!("[BBO] Bid: {} | Ask: {}", bid, ask);
                    }
                }
            })
        })
        .await?;
    println!("✓ Subscribed to BBO\n");

    // Subscribe to Trades
    println!("Subscribing to Trades channel for BTC-USD-PERP...");
    ws.lock()
        .unwrap()
        .subscribe(WebSocketChannel::Trades, Some("BTC-USD-PERP"), |message| {
            Box::pin(async move {
                if let Some(data) = message.get("data") {
                    println!("[Trade] {:?}", data);
                }
            })
        })
        .await?;
    println!("✓ Subscribed to Trades\n");

    // Subscribe to Market Summary
    println!("Subscribing to Market Summary for ALL markets...");
    ws.lock()
        .unwrap()
        .subscribe_by_name("markets_summary.ALL", |message| {
            Box::pin(async move {
                if let Some(data) = message.get("data") {
                    if let Some(symbol) = data.get("symbol") {
                        println!("[Market Summary] {}: {:?}", symbol, data);
                    }
                }
            })
        })
        .await?;
    println!("✓ Subscribed to Market Summary\n");

    // Subscribe to OrderBook snapshots
    println!("Subscribing to OrderBook for BTC-USD-PERP...");
    let orderbook_channel =
        WebSocketChannel::OrderBook.with_params(&["BTC-USD-PERP", "15", "100", "1"]);

    ws.lock()
        .unwrap()
        .subscribe_by_name(&orderbook_channel, |message| {
            Box::pin(async move {
                if let Some(data) = message.get("data") {
                    if let (Some(bids), Some(asks)) = (data.get("bids"), data.get("asks")) {
                        println!(
                            "[OrderBook] Bids: {} | Asks: {}",
                            bids.as_array().map(|a| a.len()).unwrap_or(0),
                            asks.as_array().map(|a| a.len()).unwrap_or(0)
                        );
                    }
                }
            })
        })
        .await?;
    println!("✓ Subscribed to OrderBook\n");

    println!("Listening for WebSocket messages...");
    println!("Press Ctrl+C to stop\n");
    println!("---");

    // Keep the program running
    tokio::signal::ctrl_c().await?;

    println!("\n---");
    println!("Closing WebSocket connection...");
    ws.lock().unwrap().close().await?;
    println!("✓ Disconnected");

    println!("\n=== Example completed ===");

    Ok(())
}
