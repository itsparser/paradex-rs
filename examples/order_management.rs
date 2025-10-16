use paradex_rs::{Environment, Order, OrderSide, OrderType, Paradex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize Paradex with credentials
    let _paradex = Paradex::new(Environment::Testnet)?;

    // NOTE: This example requires authentication
    // Initialize with your credentials first:
    // let paradex = Paradex::with_l1_credentials(
    //     Environment::Testnet,
    //     "your_l1_address",
    //     "your_l1_private_key"
    // )?;

    // Create an order
    let order = Order::builder()
        .market("BTC-USD-PERP")
        .side(OrderSide::Buy)
        .order_type(OrderType::Limit)
        .size("0.1")
        .price("50000")
        .build()?;

    println!("Created order: {:?}", order);

    // Submit order (requires authentication)
    // let result = paradex.api_client().submit_order(&order).await?;
    // println!("Order submitted: {:?}", result);

    Ok(())
}
