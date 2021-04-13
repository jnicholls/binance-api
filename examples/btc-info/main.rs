use std::error::Error;

use binance_api::{client::FClient, exchange::Exchange};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a Futures API client.
    let client = FClient::new();

    // Create a new Exchange API provider.
    let exchange = Exchange::new(client);

    // Get all of the exchange info for the Futures API.
    let info = exchange.info().await?;

    // Find the info for the BTC markets.
    let btc_info: Vec<_> = info
        .symbols
        .iter()
        .filter(|symbol| symbol.base_asset == "BTC")
        .collect();

    // Print out all of the BTC market information for the Futures API.
    println!("{:?}", btc_info);

    Ok(())
}
