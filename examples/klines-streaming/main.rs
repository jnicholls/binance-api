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
