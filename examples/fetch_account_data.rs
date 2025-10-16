use paradex_rs::{Environment, Paradex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize Paradex (requires authentication for private endpoints)
    let paradex = Paradex::new(Environment::Testnet)?;

    // NOTE: All endpoints below require authentication
    // Initialize with credentials first:
    // let paradex = Paradex::with_l1_credentials(
    //     Environment::Testnet,
    //     "your_l1_address",
    //     "your_l1_private_key"
    // )?;

    println!("Fetching account summary...");
    // let summary = paradex.api_client().fetch_account_summary().await?;
    // println!("Account Summary: {:?}", summary);

    println!("\nFetching positions...");
    // let positions = paradex.api_client().fetch_positions().await?;
    // println!("Positions: {:?}", positions);

    println!("\nFetching balances...");
    // let balances = paradex.api_client().fetch_balances().await?;
    // println!("Balances: {:?}", balances);

    println!("\nFetching open orders...");
    // let orders = paradex.api_client().fetch_orders(None).await?;
    // println!("Orders: {:?}", orders);

    Ok(())
}
