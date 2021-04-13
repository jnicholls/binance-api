use derive_more::Constructor;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    client::{Api, Client, FApi, SApi},
    error::Result,
    models::*,
};

pub type FTrade = Trade<FApi>;
pub type STrade = Trade<SApi>;

#[derive(Clone, Constructor, Debug)]
pub struct Trade<A: Api + TradeApi> {
    client: Client<A>,
}

impl<A> Trade<A>
where
    A: Api + TradeApi,
{
    pub async fn all_orders<S>(
        &self,
        req: AllOrdersRequest<S>,
    ) -> Result<Order<A::OrderDetails, A::OrderType>, A::ErrorCode>
    where
        S: AsRef<str>,
    {
        self.client.get(A::all_orders(), req).await
    }

    pub async fn auto_cancel_all<S>(
        &self,
        symbol: S,
        countdown_time: u64,
    ) -> Result<(), A::ErrorCode>
    where
        S: AsRef<str>,
    {
        let _: serde_json::Value = self
            .client
            .post(
                A::auto_cancel_all(),
                AutoCancelAllRequest {
                    symbol,
                    countdown_time,
                },
            )
            .await?;
        Ok(())
    }

    pub async fn cancel_all_open_orders<S>(&self, symbol: S) -> Result<(), A::ErrorCode>
    where
        S: AsRef<str>,
    {
        let symbol = Some(symbol);
        let _: serde_json::Value = self
            .client
            .delete(A::all_open_orders(), SymbolRequest { symbol })
            .await?;
        Ok(())
    }

    pub async fn cancel_batch_orders<S>(
        &self,
        symbol: S,
        order_ids: Vec<u64>,
    ) -> Result<Vec<Order<A::OrderDetails, A::OrderType>>, A::ErrorCode>
    where
        S: AsRef<str>,
    {
        self.client
            .delete(A::batch_orders(), OrderRequest::list(symbol, order_ids))
            .await
    }

    pub async fn cancel_order<S>(
        &self,
        symbol: S,
        order_id: u64,
    ) -> Result<Order<A::OrderDetails, A::OrderType>, A::ErrorCode>
    where
        S: AsRef<str>,
    {
        self.client
            .delete(A::order(), OrderRequest::single(symbol, order_id))
            .await
    }

    pub async fn leverage<S>(&self, symbol: S, leverage: u8) -> Result<Leverage, A::ErrorCode>
    where
        S: AsRef<str>,
    {
        self.client
            .post(A::leverage(), LeverageRequest { symbol, leverage })
            .await
    }

    pub async fn new_batch_orders<S>(
        &self,
        batch_orders: Vec<NewOrderRequest<A::OrderRequestDetails, A::OrderType, S>>,
    ) -> Result<Vec<BatchOrder<A::OrderDetails, A::OrderType, A::ErrorCode>>, A::ErrorCode>
    where
        S: AsRef<str>,
    {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct BatchOrders<O: Serialize> {
            batch_orders: Vec<O>,
        }

        self.client
            .post(A::batch_orders(), BatchOrders { batch_orders })
            .await
    }

    pub async fn new_order<S>(
        &self,
        req: NewOrderRequest<A::OrderRequestDetails, A::OrderType, S>,
    ) -> Result<Order<A::OrderDetails, A::OrderType>, A::ErrorCode>
    where
        S: AsRef<str>,
    {
        self.client.post(A::order(), req).await
    }

    pub async fn open_orders<S>(
        &self,
        symbol: Option<S>,
    ) -> Result<Vec<Order<A::OrderDetails, A::OrderType>>, A::ErrorCode>
    where
        S: AsRef<str>,
    {
        self.client
            .get(A::open_orders(), SymbolRequest { symbol })
            .await
    }

    pub async fn query_order<S>(
        &self,
        symbol: S,
        order_id: u64,
    ) -> Result<Order<A::OrderDetails, A::OrderType>, A::ErrorCode>
    where
        S: AsRef<str>,
    {
        self.client
            .get(A::order(), OrderRequest::single(symbol, order_id))
            .await
    }
}

pub trait TradeApi {
    type OrderRequestDetails: Serialize;
    type OrderDetails: DeserializeOwned;
    type OrderType: DeserializeOwned + Serialize;

    fn all_orders() -> &'static str;
    fn all_open_orders() -> &'static str;
    fn auto_cancel_all() -> &'static str;
    fn batch_orders() -> &'static str;
    fn leverage() -> &'static str;
    fn open_orders() -> &'static str;
    fn order() -> &'static str;
}

impl TradeApi for FApi {
    type OrderRequestDetails = FNewOrderRequest;
    type OrderDetails = FOrder;
    type OrderType = FOrderType;

    fn all_orders() -> &'static str {
        "/fapi/v1/allOrders"
    }

    fn all_open_orders() -> &'static str {
        "/fapi/v1/allOpenOrders"
    }

    fn auto_cancel_all() -> &'static str {
        "/fapi/v1/countdownCancelAll"
    }

    fn batch_orders() -> &'static str {
        "/fapi/v1/batchOrders"
    }

    fn leverage() -> &'static str {
        "/fapi/v1/leverage"
    }

    fn open_orders() -> &'static str {
        "/fapi/v1/openOrders"
    }

    fn order() -> &'static str {
        "/fapi/v1/order"
    }
}

impl TradeApi for SApi {
    type OrderRequestDetails = SNewOrderRequest;
    type OrderDetails = SOrder;
    type OrderType = SOrderType;

    fn all_orders() -> &'static str {
        "/api/v3/allOrders"
    }

    fn all_open_orders() -> &'static str {
        "/api/v3/openOrders"
    }

    fn auto_cancel_all() -> &'static str {
        unimplemented!("Spot API does not support an auto cancel API.")
    }

    fn batch_orders() -> &'static str {
        unimplemented!("Spot API does not support submitting batch orders.");
    }

    fn leverage() -> &'static str {
        unimplemented!("Spot API does not support leverage trading.");
    }

    fn open_orders() -> &'static str {
        "/api/v3/openOrders"
    }

    fn order() -> &'static str {
        "/api/v3/order"
    }
}
