use paradex_rs::{Environment, Paradex};

#[allow(clippy::await_holding_lock)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize Paradex client for testnet
    let paradex = Paradex::new(Environment::Testnet)?;

    println!("Fetching system configuration...");
    let config = paradex
        .api_client()
        .lock()
        .unwrap()
        .fetch_system_config()
        .await?;
    println!("System Config: {config:?}");

    println!("\nFetching system state...");
    let state = paradex
        .api_client()
        .lock()
        .unwrap()
        .fetch_system_state()
        .await?;
    println!("System State: {state:?}");

    println!("\nFetching markets...");
    let markets = paradex.api_client().lock().unwrap().fetch_markets().await?;
    println!("Found {} markets", markets.results.len());

    println!("\nFetching market summary...");
    let summary = paradex
        .api_client()
        .lock()
        .unwrap()
        .fetch_markets_summary(Some("BTC-USD-PERP"))
        .await?;
    println!("Market Summary: {summary:?}");

    Ok(())
}
