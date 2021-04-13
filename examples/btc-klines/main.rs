use std::error::Error;

use binance_api::{client::FClient, market::Market, models::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a Futures API client.
    let client = FClient::new();

    // Create a new Market API provider.
    let market = Market::new(client);

    // Load minutely klines for BTCUSDT from the last 1 hour.
    let one_hour_ago = chrono::Utc::now() - chrono::Duration::hours(1);
    let klines = market
        .klines(KlinesRequest::new("BTCUSDT", ChartInterval::OneMinute).start_time(one_hour_ago))
        .await?;

    // Print the klines.
    println!("{:?}", klines);

    Ok(())
}
