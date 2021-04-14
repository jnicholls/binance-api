# binance-api
An async, strongly-typed Rust library for the Binance Spot and Futures APIs.

This library is a personal project that I have decided to release publicly. There is a lot to do! *Please consider it __experimental__ at this time.*

*Note: While both Spot and Futures APIs are targeted, the ongoing maintenance of the data models are heavily biased towards the Futures API at this time.*

## Todo
* [ ] Clean up the API surface, module exports, etc.
* [ ] Documentation
* [ ] Publish on crates.io
* [ ] Unit tests with deterministic replay (e.g. something akin to Ruby's [VCR](https://github.com/vcr/vcr))
* [ ] GitHub Actions CI setup
* [ ] All the cool badges (link to docs, CI status, MSRV, etc.)
* [ ] Provide bridges to async-std and smog async runtimes

## Installation

Add to your crate a new dependency (_TODO: Update when published to crates.io_):

```toml
[dependencies]
binance-api = { git = "https://github.com/jnicholls/binance-api", branch = "main" }
```

This crate depends on [Tokio](https://tokio.rs) and expects to operate in a Tokio runtime.

## Example Usage

Print out BTC market information
```rust
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
```

Print out recent BTC minutely klines.
```rust
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
```

Stream real-time BTC and XRP klines.
```rust
use std::error::Error;

use binance_api::{models::*, ws::WSFClient};
use futures::{future, stream::StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a Futures API websocket market client.
    // Market streams are unauthenticated, public data streams.
    let (client, stream) = WSFClient::market().await?;

    // Subscribe to BTCUSDT minutely klines data in real-time as the closing price for the current minute is updated.
    client
        .send_request(
            WSRequest::new(WSRequestMethod::Subscribe)
                .stream(WSStream::Kline("BTCUSDT", ChartInterval::OneMinute)),
        )
        .await?;

    // Also subscribe to XRPUSDT.
    client
        .send_request(
            WSRequest::new(WSRequestMethod::Subscribe)
                .stream(WSStream::Kline("XRPUSDT", ChartInterval::OneMinute)),
        )
        .await?;

    // On each klines event, print out the event data.
    stream
        .for_each(|result| {
            match result {
                Ok(event) => match &event.details {
                    WSEventDetails::Kline { details } => {
                        println!("{}: {:?}", event.symbol().unwrap(), details)
                    }
                    _ => eprintln!("Unexpected event!"),
                },
                Err(e) => eprintln!("{}", e),
            }

            future::ready(())
        })
        .await;

    Ok(())
}
```
