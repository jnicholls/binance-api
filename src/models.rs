use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::str::FromStr;

use chrono::{prelude::*, serde::ts_milliseconds};
use derive_more::{Constructor, Deref, DerefMut, Display, From};
use rust_decimal::Decimal;
use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Serialize,
};
use tokio::time::Duration;

use crate::error::{ApiCode, BinanceError, Error, WSApiCode};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AggTradesRecord {
    #[serde(rename = "a")]
    pub id: u64,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "q")]
    pub quantity: Decimal,
    #[serde(rename = "f")]
    pub first_id: u64,
    #[serde(rename = "l")]
    pub last_id: u64,
    #[serde(rename = "T")]
    pub time: Time,
    #[serde(rename = "m")]
    pub buyer_is_maker: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AggTradesRequest<S>
where
    S: AsRef<str>,
{
    #[serde(serialize_with = "crate::serde::serialize_as_ref")]
    pub symbol: S,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl<S> AggTradesRequest<S>
where
    S: AsRef<str>,
{
    pub fn new(symbol: S) -> Self {
        Self {
            symbol,
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn start_time<T>(mut self, start_time: T) -> Self
    where
        T: TryInto<Time>,
    {
        self.start_time = start_time.try_into().ok();
        self
    }

    pub fn end_time<T>(mut self, end_time: T) -> Self
    where
        T: TryInto<Time>,
    {
        self.end_time = end_time.try_into().ok();
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllOrdersRequest<S>
where
    S: AsRef<str>,
{
    #[serde(serialize_with = "crate::serde::serialize_as_ref")]
    pub symbol: S,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl<S> AllOrdersRequest<S>
where
    S: AsRef<str>,
{
    pub fn new(symbol: S) -> Self {
        Self {
            symbol,
            order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn start_time<T>(mut self, start_time: T) -> Self
    where
        T: TryInto<Time>,
    {
        self.start_time = start_time.try_into().ok();
        self
    }

    pub fn end_time<T>(mut self, end_time: T) -> Self
    where
        T: TryInto<Time>,
    {
        self.end_time = end_time.try_into().ok();
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AutoCancelAllRequest<S>
where
    S: AsRef<str>,
{
    #[serde(serialize_with = "crate::serde::serialize_as_ref")]
    pub symbol: S,
    pub countdown_time: u64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub account_alias: String,
    pub asset: String,
    pub balance: Decimal,
    pub cross_wallet_balance: Decimal,
    #[serde(rename = "crossUnPnl")]
    pub cross_unrealized_profit: Decimal,
    pub available_balance: Decimal,
    pub max_withdraw_amount: Decimal,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum BatchOrder<Details, Type, C: ApiCode> {
    Order(Order<Details, Type>),
    Error(BinanceError<C>),
}

#[derive(Clone, Copy, Debug, Deserialize, Display, Serialize)]
pub enum ChartInterval {
    #[display(fmt = "1m")]
    #[serde(rename = "1m")]
    OneMinute,

    #[display(fmt = "3m")]
    #[serde(rename = "3m")]
    ThreeMinute,

    #[display(fmt = "5m")]
    #[serde(rename = "5m")]
    FiveMinute,

    #[display(fmt = "15m")]
    #[serde(rename = "15m")]
    FifteenMinute,

    #[display(fmt = "30m")]
    #[serde(rename = "30m")]
    ThirtyMinute,

    #[display(fmt = "1h")]
    #[serde(rename = "1h")]
    OneHour,

    #[display(fmt = "2h")]
    #[serde(rename = "2h")]
    TwoHour,

    #[display(fmt = "4h")]
    #[serde(rename = "4h")]
    FourHour,

    #[display(fmt = "6h")]
    #[serde(rename = "6h")]
    SixHour,

    #[display(fmt = "8h")]
    #[serde(rename = "8h")]
    EightHour,

    #[display(fmt = "12h")]
    #[serde(rename = "12h")]
    TwelveHour,

    #[display(fmt = "1d")]
    #[serde(rename = "1d")]
    OneDay,

    #[display(fmt = "3d")]
    #[serde(rename = "3d")]
    ThreeDay,

    #[display(fmt = "1w")]
    #[serde(rename = "1w")]
    OneWeek,

    #[display(fmt = "1M")]
    #[serde(rename = "1M")]
    OneMonth,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContingencyType {
    OCO,
}

#[derive(Clone, Constructor, Copy, Debug, Deserialize, Serialize)]
pub struct Empty {}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "filterType")]
pub enum ExchangeFilter {
    MaxAlgoOrders {
        #[serde(alias = "maxNumAlgoOrders")]
        limit: usize,
    },

    MaxNumOrders {
        #[serde(alias = "maxNumOrders")]
        limit: usize,
    },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo<OrderType, SymbolDetails> {
    pub exchange_filters: Vec<ExchangeFilter>,
    pub rate_limits: Vec<RateLimit>,
    pub server_time: Time,
    pub symbols: Vec<Symbol<OrderType, SymbolDetails>>,
    pub timezone: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct KlinesRecord {
    #[serde(rename = "ot")]
    pub open_time: Time,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "v")]
    pub volume: Decimal,
    #[serde(rename = "ct")]
    pub close_time: Time,
    #[serde(rename = "qav")]
    pub quote_asset_volume: Decimal,
    #[serde(rename = "n")]
    pub num_trades: usize,
    #[serde(rename = "tbbav")]
    pub taker_buy_base_asset_volume: Decimal,
    #[serde(rename = "tbqav")]
    pub taker_buy_quote_asset_volume: Decimal,
}

impl<'de> Deserialize<'de> for KlinesRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct KlinesVisitor;

        impl<'de> de::Visitor<'de> for KlinesVisitor {
            type Value = KlinesRecord;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a klines record in array format")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut index = 0;
                let mut open_time: Option<Time> = None;
                let mut open: Option<Decimal> = None;
                let mut high: Option<Decimal> = None;
                let mut low: Option<Decimal> = None;
                let mut close: Option<Decimal> = None;
                let mut volume: Option<Decimal> = None;
                let mut close_time: Option<Time> = None;
                let mut quote_asset_volume: Option<Decimal> = None;
                let mut num_trades: Option<usize> = None;
                let mut taker_buy_base_asset_volume: Option<Decimal> = None;
                let mut taker_buy_quote_asset_volume: Option<Decimal> = None;

                while let Some(v) = seq.next_element::<serde_json::Value>()? {
                    match index {
                        0 => {
                            open_time = Some(serde_json::from_value(v).map_err(de::Error::custom)?);
                        }
                        1 => {
                            open = Some(serde_json::from_value(v).map_err(de::Error::custom)?);
                        }
                        2 => {
                            high = Some(serde_json::from_value(v).map_err(de::Error::custom)?);
                        }
                        3 => {
                            low = Some(serde_json::from_value(v).map_err(de::Error::custom)?);
                        }
                        4 => {
                            close = Some(serde_json::from_value(v).map_err(de::Error::custom)?);
                        }
                        5 => {
                            volume = Some(serde_json::from_value(v).map_err(de::Error::custom)?);
                        }
                        6 => {
                            close_time =
                                Some(serde_json::from_value(v).map_err(de::Error::custom)?);
                        }
                        7 => {
                            quote_asset_volume =
                                Some(serde_json::from_value(v).map_err(de::Error::custom)?);
                        }
                        8 => {
                            num_trades =
                                Some(serde_json::from_value(v).map_err(de::Error::custom)?);
                        }
                        9 => {
                            taker_buy_base_asset_volume =
                                Some(serde_json::from_value(v).map_err(de::Error::custom)?);
                        }
                        10 => {
                            taker_buy_quote_asset_volume =
                                Some(serde_json::from_value(v).map_err(de::Error::custom)?);
                        }
                        _ => (),
                    }
                    index += 1;
                }

                if index != 12 {
                    return Err(de::Error::invalid_length(
                        index,
                        &"expected an array of 12 elements",
                    ));
                }

                Ok(KlinesRecord {
                    open_time: open_time.unwrap(),
                    open: open.unwrap(),
                    high: high.unwrap(),
                    low: low.unwrap(),
                    close: close.unwrap(),
                    volume: volume.unwrap(),
                    close_time: close_time.unwrap(),
                    quote_asset_volume: quote_asset_volume.unwrap(),
                    num_trades: num_trades.unwrap(),
                    taker_buy_base_asset_volume: taker_buy_base_asset_volume.unwrap(),
                    taker_buy_quote_asset_volume: taker_buy_quote_asset_volume.unwrap(),
                })
            }
        }

        deserializer.deserialize_seq(KlinesVisitor)
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KlinesRequest<S>
where
    S: AsRef<str>,
{
    #[serde(serialize_with = "crate::serde::serialize_as_ref")]
    pub symbol: S,
    pub interval: ChartInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl<S> KlinesRequest<S>
where
    S: AsRef<str>,
{
    pub fn new(symbol: S, interval: ChartInterval) -> Self {
        Self {
            symbol,
            interval,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn start_time<T>(mut self, start_time: T) -> Self
    where
        T: TryInto<Time>,
    {
        self.start_time = start_time.try_into().ok();
        self
    }

    pub fn end_time<T>(mut self, end_time: T) -> Self
    where
        T: TryInto<Time>,
    {
        self.end_time = end_time.try_into().ok();
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leverage {
    pub leverage: u8,
    pub max_notional_value: Decimal,
    pub symbol: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct LeverageRequest<S>
where
    S: AsRef<str>,
{
    #[serde(serialize_with = "crate::serde::serialize_as_ref")]
    pub symbol: S,
    pub leverage: u8,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarginType {
    Cross,
    Isolated,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderRequest<Details, Type, S>
where
    S: AsRef<str>,
{
    #[serde(serialize_with = "crate::serde::serialize_as_ref")]
    pub symbol: S,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub ty: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub details: Option<Details>,
}

impl<D, T, S> NewOrderRequest<D, T, S>
where
    S: AsRef<str>,
{
    pub fn new(symbol: S, side: OrderSide, ty: T) -> Self {
        Self {
            symbol,
            side,
            ty,
            time_in_force: None,
            quantity: None,
            price: None,
            new_client_order_id: None,
            stop_price: None,
            new_order_resp_type: None,
            details: None,
        }
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    pub fn quantity(mut self, quantity: Decimal) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn price(mut self, price: Decimal) -> Self {
        self.price = Some(price);
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: String) -> Self {
        self.new_client_order_id = Some(new_client_order_id);
        self
    }

    pub fn stop_price(mut self, stop_price: Decimal) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: OrderResponseType) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    pub fn details(mut self, details: D) -> Self {
        self.details = Some(details);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FNewOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_rate: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_type: Option<WorkingType>,
}

impl FNewOrderRequest {
    pub fn position_side(mut self, position_side: PositionSide) -> Self {
        self.position_side = Some(position_side);
        self
    }

    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    pub fn close_position(mut self, close_position: String) -> Self {
        self.close_position = Some(close_position);
        self
    }

    pub fn activation_price(mut self, activation_price: Decimal) -> Self {
        self.activation_price = Some(activation_price);
        self
    }

    pub fn callback_rate(mut self, callback_rate: Decimal) -> Self {
        self.callback_rate = Some(callback_rate);
        self
    }

    pub fn working_type(mut self, working_type: WorkingType) -> Self {
        self.working_type = Some(working_type);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SNewOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<Decimal>,
}

impl SNewOrderRequest {
    pub fn quote_order_qty(mut self, quote_order_qty: Decimal) -> Self {
        self.quote_order_qty = Some(quote_order_qty);
        self
    }

    pub fn iceberg_qty(mut self, iceberg_qty: Decimal) -> Self {
        self.iceberg_qty = Some(iceberg_qty);
        self
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OCOStatus {
    Response,
    ExecStarted,
    AllDone,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OCOOrderStatus {
    Executing,
    AllDone,
    Reject,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,

    #[serde(alias = "E", default)]
    pub message_output_time: Option<Time>,
    #[serde(alias = "T", default)]
    pub transaction_time: Option<Time>,

    // (price, quantity)
    pub bids: Vec<(Decimal, Decimal)>,
    pub asks: Vec<(Decimal, Decimal)>,
}

#[derive(Clone, Debug, Serialize)]
pub struct OrderBookRequest<S>
where
    S: AsRef<str>,
{
    #[serde(serialize_with = "crate::serde::serialize_as_ref")]
    pub symbol: S,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl<S> OrderBookRequest<S>
where
    S: AsRef<str>,
{
    pub fn new(symbol: S) -> Self {
        Self {
            symbol,
            limit: None,
        }
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order<Details, Type> {
    pub client_order_id: String,
    pub executed_qty: Decimal,
    pub order_id: u64,
    pub orig_qty: Decimal,
    pub price: Decimal,
    pub side: OrderSide,
    pub status: OrderStatus,
    pub symbol: String,
    #[serde(default, alias = "transactTime", alias = "updateTime")]
    pub time: Option<Time>,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub ty: Type,

    #[serde(flatten)]
    pub details: Details,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FOrder {
    pub avg_price: Decimal,
    pub cum_qty: Decimal,
    pub cum_quote: Decimal,
    pub orig_type: FOrderType,
    pub reduce_only: bool,
    pub position_side: PositionSide,
    pub stop_price: Decimal,
    pub close_position: bool,
    #[serde(default)]
    pub activate_price: Decimal,
    #[serde(default)]
    pub price_rate: Decimal,
    pub working_type: WorkingType,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SOrder {
    pub cummulative_quote_qty: Decimal,
    pub order_list_id: i64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OrderRequest<S>
where
    S: AsRef<str>,
{
    #[serde(serialize_with = "crate::serde::serialize_as_ref")]
    symbol: S,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<u64>,
    #[serde(
        serialize_with = "crate::serde::serialize_json",
        skip_serializing_if = "Option::is_none"
    )]
    order_id_list: Option<Vec<u64>>,
}

impl<S> OrderRequest<S>
where
    S: AsRef<str>,
{
    pub fn single(symbol: S, order_id: u64) -> Self {
        let order_id = Some(order_id);
        let order_id_list = None;
        Self {
            symbol,
            order_id,
            order_id_list,
        }
    }

    pub fn list(symbol: S, order_id_list: Vec<u64>) -> Self {
        let order_id = None;
        let order_id_list = Some(order_id_list);
        Self {
            symbol,
            order_id,
            order_id_list,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderResponseType {
    Ack,
    Result,
    Full,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
    NewInsurance,
    NewAdl,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FOrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
    Liquidation,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SOrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub entry_price: Decimal,
    pub margin_type: MarginType,
    #[serde(deserialize_with = "crate::serde::deserialize_string_as_bool")]
    pub is_auto_add_margin: bool,
    pub isolated_margin: Decimal,
    pub leverage: Decimal,
    pub liquidation_price: Decimal,
    pub mark_price: Decimal,
    pub max_notional_value: Decimal,
    pub position_amt: Decimal,
    pub symbol: String,
    #[serde(rename = "unRealizedProfit")]
    pub unrealized_profit: Decimal,
    pub position_side: PositionSide,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RateLimit {
    pub interval: RateLimitInterval,
    #[serde(rename = "intervalNum")]
    pub num: u8,
    pub limit: u32,
    #[serde(rename = "rateLimitType")]
    pub ty: RateLimitType,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Second,
    Minute,
    Day,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RequestWeight,
    Orders,
    RawRequests,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    PreTrading,
    PendingTrading,
    Trading,
    PostTrading,
    EndOfDay,
    Halt,
    AuctionMatch,
    Break,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol<OrderType, SymbolDetails> {
    pub symbol: String,
    pub status: Status,
    pub base_asset: String,
    pub quote_asset: String,
    pub base_asset_precision: u8,
    pub quote_precision: u8,
    pub order_types: Vec<OrderType>,
    pub filters: Vec<SymbolFilter>,

    #[serde(flatten)]
    pub details: SymbolDetails,
}

impl<O, S> AsRef<str> for Symbol<O, S> {
    fn as_ref(&self) -> &str {
        &self.symbol
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FSymbol {
    pub margin_asset: String,
    pub maint_margin_percent: Decimal,
    pub required_margin_percent: Decimal,
    pub price_precision: u8,
    pub quantity_precision: u8,
    pub time_in_force: Vec<TimeInForce>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SSymbol {
    pub quote_asset_precision: u8,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub permissions: Vec<Type>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "filterType")]
pub enum SymbolFilter {
    IcebergParts {
        #[serde(alias = "icebergParts")]
        limit: usize,
    },

    #[serde(rename_all = "camelCase")]
    LotSize {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },

    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },

    MaxNumAlgoOrders {
        #[serde(alias = "maxNumAlgoOrders")]
        limit: usize,
    },

    MaxNumIcebergOrders {
        #[serde(alias = "maxNumIcebergOrders")]
        limit: usize,
    },

    MaxNumOrders {
        #[serde(alias = "maxNumOrders")]
        limit: usize,
    },

    MaxPosition {
        #[serde(alias = "maxPosition")]
        limit: Decimal,
    },

    MinNotional {
        #[serde(alias = "minNotional")]
        notional: Decimal,
    },

    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: Decimal,
        multiplier_down: Decimal,

        // Only present in Spot API.
        #[serde(default)]
        avg_price_mins: Option<u32>,

        // Only present in Futures API.
        #[serde(default)]
        multiplier_decimal: Option<String>,
    },

    #[serde(rename_all = "camelCase")]
    PriceFilter {
        min_price: Decimal,
        max_price: Decimal,
        tick_size: Decimal,
    },
}

#[derive(Clone, Debug, Serialize)]
pub struct SymbolRequest<S>
where
    S: AsRef<str>,
{
    #[serde(
        serialize_with = "crate::serde::serialize_optional_as_ref",
        skip_serializing_if = "Option::is_none"
    )]
    pub symbol: Option<S>,
}

#[derive(
    Clone,
    Constructor,
    Copy,
    Debug,
    Deref,
    DerefMut,
    Deserialize,
    Display,
    Eq,
    From,
    PartialEq,
    PartialOrd,
    Ord,
    Serialize,
)]
#[serde(transparent)]
pub struct Time(#[serde(with = "ts_milliseconds")] pub DateTime<Utc>);

impl Default for Time {
    fn default() -> Self {
        Time(Utc.timestamp(0, 0))
    }
}

impl FromStr for Time {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> chrono::ParseResult<Self> {
        s.parse().map(|dt: DateTime<Utc>| Time(dt))
    }
}

macro_rules! time_from_str {
    ( $( $t:ty ),* ) => {
        $(
            impl TryFrom<$t> for Time {
                type Error = chrono::ParseError;

                fn try_from(s: $t) -> chrono::ParseResult<Self> {
                    s.parse()
                }
            }
        )*
    }
}

time_from_str!(&str, String, &String);

impl TryFrom<i64> for Time {
    type Error = ();

    fn try_from(s: i64) -> Result<Self, Self::Error> {
        match Utc.timestamp_millis_opt(s) {
            chrono::LocalResult::Single(dt) => Ok(Time(dt)),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    GoodTilCancel,
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
    #[serde(rename = "FOK")]
    FillOrKill,
    #[serde(rename = "GTX")]
    GoodTilCrossing,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    Future,
    Leveraged,
    Margin,
    Spot,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEvent<OrderType> {
    #[serde(alias = "E")]
    pub time: Time,
    #[serde(alias = "s", default)]
    pub symbol: Option<String>,
    #[serde(flatten)]
    pub details: WSEventDetails<OrderType>,
}

impl<OrderType> WSEvent<OrderType> {
    pub fn symbol(&self) -> Option<&str> {
        self.symbol.as_deref().or_else(|| match &self.details {
            WSEventDetails::ForceOrder {
                details: WSEventForceOrder { symbol, .. },
            } => Some(symbol.as_str()),
            WSEventDetails::OrderUpdate {
                details: WSEventOrderUpdate { symbol, .. },
                ..
            } => Some(symbol.as_str()),
            _ => None,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "e")]
pub enum WSEventDetails<OrderType> {
    #[serde(alias = "ACCOUNT_UPDATE")]
    AccountUpdate {
        #[serde(alias = "T")]
        transaction_time: Time,
        #[serde(alias = "a")]
        details: WSEventAccountUpdate,
    },
    AggTrade(WSEventAggTrade),
    BookTicker(WSEventBookTicker),
    ForceOrder {
        #[serde(alias = "o")]
        details: WSEventForceOrder<OrderType>,
    },
    Kline {
        #[serde(alias = "k")]
        details: WSEventKline,
    },
    ListenKeyExpired,
    #[serde(alias = "markPriceUpdate")]
    MarkPrice(WSEventMarkPrice),
    #[serde(alias = "MARGIN_CALL")]
    MarginCall {
        #[serde(alias = "cw")]
        cross_wallet_balance: Decimal,
        #[serde(alias = "p")]
        positions: Vec<WSEventMarginCallPosition>,
    },
    #[serde(alias = "24hrMiniTicker")]
    MiniTicker(WSEventMiniTicker),
    #[serde(alias = "ORDER_TRADE_UPDATE")]
    OrderUpdate {
        #[serde(alias = "T")]
        transaction_time: Time,
        #[serde(alias = "o")]
        details: WSEventOrderUpdate<OrderType>,
    },
    #[serde(alias = "depthUpdate")]
    OrderBookUpdate(WSEventOrderBookUpdate),
    #[serde(alias = "24hrTicker")]
    Ticker(WSEventTicker),
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventAccountUpdate {
    #[serde(alias = "m")]
    pub update_type: WSEventAccountUpdateType,
    #[serde(alias = "B", default)]
    pub balances: Vec<WSEventAccountUpdateBalance>,
    #[serde(alias = "P", default)]
    pub positions: Vec<WSEventAccountUpdatePosition>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventAccountUpdateBalance {
    #[serde(alias = "a")]
    pub asset: String,
    #[serde(alias = "wb")]
    pub wallet_balance: Decimal,
    #[serde(alias = "cw")]
    pub cross_wallet_balance: Decimal,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventAccountUpdatePosition {
    #[serde(alias = "s")]
    pub symbol: String,
    #[serde(alias = "pa")]
    pub amount: Decimal,
    #[serde(alias = "ep")]
    pub entry_price: Decimal,
    #[serde(alias = "cr")]
    pub accumulated_realized: Decimal,
    #[serde(alias = "up")]
    pub unrealized_profit: Decimal,
    #[serde(alias = "mt")]
    pub margin_type: MarginType,
    #[serde(alias = "iw")]
    pub isolated_wallet: Decimal,
    #[serde(alias = "ps")]
    pub position_side: PositionSide,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WSEventAccountUpdateType {
    Deposit,
    Withdraw,
    Order,
    FundingFee,
    WithdrawReject,
    Adjustment,
    InsuranceClear,
    AdminDeposit,
    AdminWithdraw,
    MarginTransfer,
    MarginTypeChange,
    AssetTransfer,
    OptionsPremiumFee,
    OptionsSettleProfit,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventAggTrade {
    #[serde(alias = "a")]
    pub id: u64,
    #[serde(alias = "p")]
    pub price: Decimal,
    #[serde(alias = "q")]
    pub quantity: Decimal,
    #[serde(alias = "f")]
    pub first_id: u64,
    #[serde(alias = "l")]
    pub last_id: u64,
    #[serde(alias = "T")]
    pub trade_time: Time,
    #[serde(alias = "m")]
    pub buyer_is_maker: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventBookTicker {
    #[serde(alias = "u")]
    pub update_id: u64,
    #[serde(alias = "T")]
    pub transaction_time: Time,
    #[serde(alias = "b")]
    pub best_bid_price: Decimal,
    #[serde(alias = "B")]
    pub best_bid_qty: Decimal,
    #[serde(alias = "a")]
    pub best_ask_price: Decimal,
    #[serde(alias = "A")]
    pub best_ask_qty: Decimal,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventForceOrder<OrderType> {
    #[serde(alias = "s")]
    pub symbol: String,
    #[serde(alias = "S")]
    pub side: OrderSide,
    #[serde(alias = "o")]
    pub ty: OrderType,
    #[serde(alias = "f")]
    pub time_in_force: TimeInForce,
    #[serde(alias = "q")]
    pub original_qty: Decimal,
    #[serde(alias = "p")]
    pub price: Decimal,
    #[serde(alias = "ap")]
    pub avg_price: Decimal,
    #[serde(alias = "X")]
    pub status: OrderStatus,
    #[serde(alias = "l")]
    pub last_filled_qty: Decimal,
    #[serde(alias = "z")]
    pub accumulated_filled_qty: Decimal,
    #[serde(alias = "T")]
    pub trade_time: Time,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventKline {
    #[serde(alias = "t")]
    pub start_time: Time,
    #[serde(alias = "T")]
    pub close_time: Time,
    #[serde(alias = "i")]
    pub interval: ChartInterval,
    #[serde(alias = "f")]
    pub first_id: u64,
    #[serde(alias = "L")]
    pub last_id: u64,
    #[serde(alias = "o")]
    pub open: Decimal,
    #[serde(alias = "c")]
    pub close: Decimal,
    #[serde(alias = "h")]
    pub high: Decimal,
    #[serde(alias = "l")]
    pub low: Decimal,
    #[serde(alias = "v")]
    pub volume: Decimal,
    #[serde(alias = "n")]
    pub num_trades: usize,
    #[serde(alias = "x")]
    pub is_closed: bool,
    #[serde(alias = "q")]
    pub quote_asset_volume: Decimal,
    #[serde(alias = "V")]
    pub taker_buy_base_asset_volume: Decimal,
    #[serde(alias = "Q")]
    pub taker_buy_quote_asset_volume: Decimal,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventMarkPrice {
    #[serde(alias = "p")]
    pub price: Decimal,
    #[serde(alias = "q")]
    pub quantity: Decimal,
    #[serde(alias = "i")]
    pub index_price: Decimal,
    #[serde(alias = "r")]
    pub funding_rate: Decimal,
    #[serde(alias = "T")]
    pub next_funding_time: Time,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventMarginCallPosition {
    #[serde(alias = "s")]
    pub symbol: String,
    #[serde(alias = "ps")]
    pub position_side: PositionSide,
    #[serde(alias = "pa")]
    pub position_amt: Decimal,
    #[serde(alias = "mt")]
    pub margin_type: MarginType,
    #[serde(alias = "iw")]
    pub isolated_wallet: Decimal,
    #[serde(alias = "mp")]
    pub mark_price: Decimal,
    #[serde(alias = "up")]
    pub unrealized_profit: Decimal,
    #[serde(alias = "mm")]
    pub required_maint_margin: Decimal,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventMiniTicker {
    #[serde(alias = "c")]
    pub close: Decimal,
    #[serde(alias = "o")]
    pub open: Decimal,
    #[serde(alias = "h")]
    pub high: Decimal,
    #[serde(alias = "l")]
    pub low: Decimal,
    #[serde(alias = "v")]
    pub base_asset_volume: Decimal,
    #[serde(alias = "q")]
    pub quote_asset_volume: Decimal,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventOrderUpdate<OrderType> {
    #[serde(alias = "s")]
    pub symbol: String,
    #[serde(alias = "c")]
    pub client_order_id: String,
    #[serde(alias = "S")]
    pub side: OrderSide,
    #[serde(alias = "o")]
    pub ty: OrderType,
    #[serde(alias = "f")]
    pub time_in_force: TimeInForce,
    #[serde(alias = "q")]
    pub original_qty: Decimal,
    #[serde(alias = "p")]
    pub original_price: Decimal,
    #[serde(alias = "ap")]
    pub avg_price: Decimal,
    #[serde(alias = "sp")]
    pub stop_price: Decimal,
    #[serde(alias = "x")]
    pub execution_type: WSEventOrderUpdateExecType,
    #[serde(alias = "X")]
    pub status: OrderStatus,
    #[serde(alias = "i")]
    pub order_id: u64,
    #[serde(alias = "l")]
    pub last_filled_qty: Decimal,
    #[serde(alias = "z")]
    pub accumulated_filled_qty: Decimal,
    #[serde(alias = "L")]
    pub last_filled_price: Decimal,
    #[serde(alias = "N", default)]
    pub commission_asset: String,
    #[serde(alias = "n", default)]
    pub commission: Decimal,
    #[serde(alias = "T")]
    pub trade_time: Time,
    #[serde(alias = "t")]
    pub trade_id: u64,
    #[serde(alias = "b")]
    pub bids_notional: Decimal,
    #[serde(alias = "a")]
    pub asks_notional: Decimal,
    #[serde(alias = "m")]
    pub is_maker: bool,
    #[serde(alias = "R")]
    pub reduce_only: bool,
    #[serde(alias = "wt")]
    pub stop_price_working_type: WorkingType,
    #[serde(alias = "ot")]
    pub original_order_type: OrderType,
    #[serde(alias = "ps")]
    pub position_side: PositionSide,
    #[serde(alias = "cp")]
    pub conditional_close_all: bool,
    #[serde(alias = "AP", default)]
    pub activation_price: Decimal,
    #[serde(alias = "cr", default)]
    pub callback_rate: Decimal,
    #[serde(alias = "rp")]
    pub realized_profit: Decimal,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WSEventOrderUpdateExecType {
    New,
    PartialFill,
    Fill,
    Canceled,
    Calculated,
    Expired,
    Trade,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventOrderBookUpdate {
    #[serde(alias = "T")]
    pub transaction_time: Time,
    #[serde(alias = "U")]
    pub first_id: u64,
    #[serde(alias = "u")]
    pub last_id: u64,
    #[serde(alias = "pu")]
    pub prev_last_id: u64,

    // (price, quantity)
    #[serde(alias = "b")]
    pub bids: Vec<(Decimal, Decimal)>,
    #[serde(alias = "a")]
    pub asks: Vec<(Decimal, Decimal)>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSEventTicker {
    #[serde(alias = "p")]
    pub price_change: Decimal,
    #[serde(alias = "P")]
    pub price_change_percent: Decimal,
    #[serde(alias = "w")]
    pub weighted_avg_price: Decimal,
    #[serde(alias = "c")]
    pub last_price: Decimal,
    #[serde(alias = "Q")]
    pub last_quantity: Decimal,
    #[serde(alias = "o")]
    pub open: Decimal,
    #[serde(alias = "h")]
    pub high: Decimal,
    #[serde(alias = "l")]
    pub low: Decimal,
    #[serde(alias = "v")]
    pub base_asset_volume: Decimal,
    #[serde(alias = "q")]
    pub quote_asset_volume: Decimal,
    #[serde(alias = "O")]
    pub stat_open_time: Time,
    #[serde(alias = "C")]
    pub stat_close_time: Time,
    #[serde(alias = "F")]
    pub first_id: u64,
    #[serde(alias = "L")]
    pub last_id: u64,
    #[serde(alias = "n")]
    pub num_trades: usize,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum WSMessage<OrderType> {
    Event(WSEvent<OrderType>),
    Request(WSRequest),
    Response(WSResponse),
    Pong(Vec<u8>),
}

pub const WS_PROPERTY_COMBINED: &str = "combined";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WSRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<u64>,
    method: WSRequestMethod,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    params: Vec<serde_json::Value>,
    #[serde(skip)]
    pub(crate) timeout: Option<Duration>,
}

impl WSRequest {
    pub fn new(method: WSRequestMethod) -> Self {
        Self {
            id: None,
            method,
            params: Vec::new(),
            timeout: None,
        }
    }

    pub fn get_property<S>(mut self, property: S) -> Self
    where
        S: AsRef<str>,
    {
        self.method = WSRequestMethod::GetProperty;
        self.params
            .push(serde_json::Value::String(property.as_ref().to_owned()));
        self
    }

    pub fn set_property<S, T>(mut self, property: S, value: T) -> Self
    where
        S: AsRef<str>,
        T: Serialize,
    {
        self.method = WSRequestMethod::SetProperty;
        self.params
            .push(serde_json::Value::String(property.as_ref().to_owned()));
        let value = serde_json::to_value(value).unwrap_or_default();
        self.params.push(value);
        self
    }

    pub fn stream<S>(mut self, stream: WSStream<S>) -> Self
    where
        S: AsRef<str>,
    {
        self.params
            .push(serde_json::Value::String(stream.to_string()));
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WSRequestMethod {
    Subscribe,
    Unsubscribe,
    ListSubscriptions,
    SetProperty,
    GetProperty,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WSResponse {
    pub id: u64,
    result: serde_json::Value,
}

impl WSResponse {
    pub fn result<T>(&self) -> Result<T, Error<WSApiCode>>
    where
        T: DeserializeOwned,
    {
        Ok(serde_json::from_value(self.result.clone())?)
    }
}

#[derive(Clone, Debug, Display)]
pub enum WSStream<S>
where
    S: AsRef<str>,
{
    #[display(fmt = "{}@aggTrade", "_0.as_ref().to_lowercase()")]
    AggTrade(S),
    #[display(fmt = "!bookTicker")]
    AllBookTicker,
    #[display(fmt = "!forceOrder@arr")]
    AllForceLiquidationOrder,
    #[display(fmt = "!markPrice@arr")]
    AllMarkPrice,
    #[display(fmt = "!markPrice@arr@1s")]
    AllMarkPriceOneSec,
    #[display(fmt = "!miniTicker@arr")]
    AllMiniTicker,
    #[display(fmt = "!ticker@arr")]
    AllTicker,
    #[display(fmt = "{}@depth", "_0.as_ref().to_lowercase()")]
    BookDepth(S),
    #[display(fmt = "{}@depth@500ms", "_0.as_ref().to_lowercase()")]
    BookDepth500ms(S),
    #[display(fmt = "{}@depth@100ms", "_0.as_ref().to_lowercase()")]
    BookDepth100ms(S),
    #[display(fmt = "{}@depth@0ms", "_0.as_ref().to_lowercase()")]
    BookDepthRealTime(S),
    #[display(fmt = "{}@bookTicker", "_0.as_ref().to_lowercase()")]
    BookTicker(S),
    #[display(fmt = "{}@forceOrder", "_0.as_ref().to_lowercase()")]
    ForceLiquidationOrder(S),
    #[display(fmt = "{}@kline_{}", "_0.as_ref().to_lowercase()", "_1")]
    Kline(S, ChartInterval),
    #[display(fmt = "{}@markPrice", "_0.as_ref().to_lowercase()")]
    MarkPrice(S),
    #[display(fmt = "{}@markPrice@1s", "_0.as_ref().to_lowercase()")]
    MarkPriceOneSec(S),
    #[display(fmt = "{}@miniTicker", "_0.as_ref().to_lowercase()")]
    MiniTicker(S),
    #[display(fmt = "{}@depth{}", "_0.as_ref().to_lowercase()", "_1")]
    PartialBookDepth(S, u8),
    #[display(fmt = "{}@depth{}@500ms", "_0.as_ref().to_lowercase()", "_1")]
    PartialBookDepth500ms(S, u8),
    #[display(fmt = "{}@depth{}@100ms", "_0.as_ref().to_lowercase()", "_1")]
    PartialBookDepth100ms(S, u8),
    #[display(fmt = "{}@ticker", "_0.as_ref().to_lowercase()")]
    Ticker(S),
    #[display(fmt = "{}", "_0.as_ref()")]
    UserData(S),
}
