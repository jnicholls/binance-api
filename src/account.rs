use std::collections::HashMap;

use derive_more::Constructor;
use serde::Deserialize;

use crate::{
    client::{Api, Client, FApi, SApi},
    error::Result,
    models::*,
};

#[derive(Clone, Constructor, Debug)]
pub struct Account<A: Api + AccountApi> {
    client: Client<A>,
}

impl<A> Account<A>
where
    A: Api + AccountApi,
{
    pub async fn balance(&self) -> Result<HashMap<String, Balance>, A::ErrorCode> {
        self.client
            .get(A::balance(), Empty::new())
            .await
            .map(|balances: Vec<Balance>| {
                balances.into_iter().map(|b| (b.asset.clone(), b)).collect()
            })
    }

    pub async fn hedge_mode(&self) -> Result<bool, A::ErrorCode> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct HedgeMode {
            dual_side_position: bool,
        }

        self.client
            .get::<_, HedgeMode>(A::hedge_mode(), Empty::new())
            .await
            .map(|hm| hm.dual_side_position)
    }

    pub async fn listen_key(&self) -> Result<String, A::ErrorCode> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ListenKey {
            listen_key: String,
        }

        self.client
            .post::<_, ListenKey>(A::listen_key(), Empty::new())
            .await
            .map(|lk| lk.listen_key)
    }

    pub async fn listen_key_close(&self) -> Result<Empty, A::ErrorCode> {
        self.client.delete(A::listen_key(), Empty::new()).await
    }

    pub async fn listen_key_keepalive(&self) -> Result<Empty, A::ErrorCode> {
        self.client.put(A::listen_key(), Empty::new()).await
    }

    pub async fn positions<S>(&self, symbol: Option<S>) -> Result<Vec<Position>, A::ErrorCode>
    where
        S: AsRef<str>,
    {
        self.client
            .get(A::positions(), SymbolRequest { symbol })
            .await
    }

    pub async fn set_hedge_mode(&self, hedge_mode: bool) -> Result<(), A::ErrorCode> {
        let _ = self
            .client
            .post::<_, serde_json::Value>(A::hedge_mode(), [("dualSidePosition", hedge_mode)])
            .await;
        Ok(())
    }
}

pub trait AccountApi {
    fn balance() -> &'static str;
    fn hedge_mode() -> &'static str;
    fn listen_key() -> &'static str;
    fn positions() -> &'static str;
}

impl AccountApi for FApi {
    fn balance() -> &'static str {
        "/fapi/v2/balance"
    }

    fn hedge_mode() -> &'static str {
        "/fapi/v1/positionSide/dual"
    }

    fn listen_key() -> &'static str {
        "/fapi/v1/listenKey"
    }

    fn positions() -> &'static str {
        "/fapi/v2/positionRisk"
    }
}

impl AccountApi for SApi {
    fn balance() -> &'static str {
        unimplemented!("Spot API does not support a balance API.");
    }

    fn hedge_mode() -> &'static str {
        unimplemented!("Spot API does not support the notion of positions.");
    }

    fn listen_key() -> &'static str {
        "/api/v3/listenKey"
    }

    fn positions() -> &'static str {
        unimplemented!("Spot API does not support the notion of positions.");
    }
}

pub type FAccount = Account<FApi>;
pub type SAccount = Account<SApi>;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{AuthKeys, WSFClient};
//     use futures::{future, stream::StreamExt};
//     use tokio::time;

//     fn api_keys() -> (String, String) {
//         use std::env;

//         (
//             env::var("TRADER_API_KEY").unwrap(),
//             env::var("TRADER_API_SECRET").unwrap(),
//         )
//     }

//     #[tokio::test]
//     async fn balance() {
//         let (key, secret) = api_keys();
//         let auth_keys = AuthKeys::new(key, secret);
//         let client = Client::<FApi>::new(Some(auth_keys));
//         let account = Account::new(client);

//         eprintln!("{:?}", account.balance().await);
//     }

//     #[tokio::test]
//     async fn hedge_mode() {
//         let (key, secret) = api_keys();
//         let auth_keys = AuthKeys::new(key, secret);
//         let client = Client::<FApi>::new(Some(auth_keys));
//         let account = Account::new(client);

//         eprintln!("{:?}", account.hedge_mode().await);
//         eprintln!("{:?}", account.set_hedge_mode(true).await);
//     }

//     #[tokio::test]
//     async fn listen_key() {
//         let (key, secret) = api_keys();
//         let auth_keys = AuthKeys::new(key, secret);
//         let client = Client::<FApi>::new(Some(auth_keys));
//         let account = Account::new(client);
//         let listen_key = account.listen_key().await.unwrap();

//         let (client, stream) = WSFClient::user_data(listen_key).await.unwrap();
//         {
//             tokio::spawn(async move {
//                 time::sleep(time::Duration::from_secs(10)).await;
//                 client.close();
//             });
//         }
//         stream
//             .for_each(|e| {
//                 if let Ok(e) = e {
//                     eprintln!("{:?}", e);
//                 }
//                 future::ready(())
//             })
//             .await;
//     }

//     #[tokio::test]
//     async fn positions() {
//         let (key, secret) = api_keys();
//         let auth_keys = AuthKeys::new(key, secret);
//         let client = Client::<FApi>::new(Some(auth_keys));
//         let account = Account::new(client);

//         eprintln!("{:?}", account.positions(Some("ETHUSDT")).await);
//     }
// }
