use derive_more::Constructor;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{
    client::{Api, Client, FApi, SApi},
    error::Result,
    models::*,
};

#[derive(Clone, Constructor, Debug)]
pub struct Exchange<A: Api + ExchangeApi> {
    client: Client<A>,
}

impl<A> Exchange<A>
where
    A: Api + ExchangeApi,
{
    pub async fn info(&self) -> Result<ExchangeInfo<A::OrderType, A::SymbolDetails>, A::ErrorCode> {
        self.client.get(A::info(), Empty::new()).await
    }

    pub async fn ping(&self) -> Result<Empty, A::ErrorCode> {
        self.client.get(A::ping(), Empty::new()).await
    }

    pub async fn time(&self) -> Result<Time, A::ErrorCode> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ServerTime {
            server_time: Time,
        }

        self.client
            .get::<_, ServerTime>(A::time(), Empty::new())
            .await
            .map(|st| st.server_time)
    }
}

pub trait ExchangeApi {
    type OrderType: DeserializeOwned;
    type SymbolDetails: DeserializeOwned;

    fn info() -> &'static str;
    fn ping() -> &'static str;
    fn time() -> &'static str;
}

impl ExchangeApi for FApi {
    type OrderType = FOrderType;
    type SymbolDetails = FSymbol;

    fn info() -> &'static str {
        "/fapi/v1/exchangeInfo"
    }

    fn ping() -> &'static str {
        "/fapi/v1/ping"
    }

    fn time() -> &'static str {
        "/fapi/v1/time"
    }
}

impl ExchangeApi for SApi {
    type OrderType = SOrderType;
    type SymbolDetails = SSymbol;

    fn info() -> &'static str {
        "/api/v3/exchangeInfo"
    }

    fn ping() -> &'static str {
        "/api/v3/ping"
    }

    fn time() -> &'static str {
        "/api/v3/time"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn info() {
        eprintln!("{:?}", Exchange::new(Client::<FApi>::new()).info().await);
    }
}
