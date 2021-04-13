use derive_more::Constructor;

use crate::{
    client::{Api, Client, FApi, SApi},
    error::Result,
    models::*,
};

#[derive(Clone, Constructor, Debug)]
pub struct Market<A: Api + MarketApi> {
    client: Client<A>,
}

impl<A> Market<A>
where
    A: Api + MarketApi,
{
    pub async fn agg_trades<S>(
        &self,
        req: AggTradesRequest<S>,
    ) -> Result<Vec<AggTradesRecord>, A::ErrorCode>
    where
        S: AsRef<str>,
    {
        self.client.get(A::agg_trades(), req).await
    }

    pub async fn klines<S>(&self, req: KlinesRequest<S>) -> Result<Vec<KlinesRecord>, A::ErrorCode>
    where
        S: AsRef<str>,
    {
        self.client.get(A::klines(), req).await
    }

    pub async fn order_book<S>(&self, req: OrderBookRequest<S>) -> Result<OrderBook, A::ErrorCode>
    where
        S: AsRef<str>,
    {
        self.client.get(A::order_book(), req).await
    }
}

pub trait MarketApi {
    fn agg_trades() -> &'static str;
    fn klines() -> &'static str;
    fn order_book() -> &'static str;
}

impl MarketApi for FApi {
    fn agg_trades() -> &'static str {
        "/fapi/v1/aggTrades"
    }

    fn klines() -> &'static str {
        "/fapi/v1/klines"
    }

    fn order_book() -> &'static str {
        "/fapi/v1/depth"
    }
}

impl MarketApi for SApi {
    fn agg_trades() -> &'static str {
        "/api/v3/aggTrades"
    }

    fn klines() -> &'static str {
        "/api/v3/klines"
    }

    fn order_book() -> &'static str {
        "/api/v3/depth"
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::Exchange;

//     #[tokio::test]
//     async fn agg_trades() {
//         let client = Client::<FApi>::new(None);
//         let exchange = Exchange::new(client.clone());
//         let market = Market::new(client);

//         let info = exchange.info().await.unwrap();
//         let btc = info.symbols.iter().find(|s| s.symbol == "BTCUSDT").unwrap();
//         let trades = market.agg_trades(AggTradesRequest::new(btc)).await.unwrap();

//         eprintln!("{:?}", trades);
//     }

//     #[tokio::test]
//     async fn klines() {
//         let client = Client::<FApi>::new(None);
//         let exchange = Exchange::new(client.clone());
//         let market = Market::new(client);

//         let info = exchange.info().await.unwrap();
//         let btc = info.symbols.iter().find(|s| s.symbol == "BTCUSDT").unwrap();
//         let klines = market
//             .klines(
//                 KlinesRequest::new(btc, ChartInterval::OneMinute)
//                     .start_time("2020-06-01T00:00:00Z")
//                     .end_time("2020-06-01T07:59:59Z"),
//             )
//             .await
//             .unwrap();

//         eprintln!("{:?}", klines);
//     }

//     #[tokio::test]
//     async fn order_book() {
//         let client = Client::<SApi>::new(None);
//         let exchange = Exchange::new(client.clone());
//         let market = Market::new(client);

//         let info = exchange.info().await.unwrap();
//         let btc = info.symbols.iter().find(|s| s.symbol == "BTCUSDT").unwrap();
//         let order_book = market.order_book(OrderBookRequest::new(btc)).await.unwrap();

//         eprintln!("{:?}", order_book);
//     }
// }
