use std::clone::Clone;
use std::convert::TryFrom;
use std::fmt;
use std::marker::{Copy, PhantomData};
use std::result;

use derive_more::Constructor;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{de, Deserialize};
use serde_repr::Deserialize_repr;

pub type Result<T, C> = result::Result<T, Error<C>>;

#[derive(Clone, Copy, Debug, Deserialize_repr, FromPrimitive)]
#[repr(i16)]
pub enum CommonCode {
    // 10xx General server or network issues
    Unknown = -1000,
    Disconnected = -1001,
    Unauthorized = -1002,
    TooManyRequests = -1003,
    DuplicateIp = -1004,
    NoSuchIp = -1005,
    UnexpectedResponse = -1006,
    Timeout = -1007,
    ErrorMessageReceived = -1010,
    IpNotOnWhiteList = -1011,
    InvalidMessage = -1013,
    UnknownOrderComposition = -1014,
    TooManyOrders = -1015,
    ServiceShuttingDown = -1016,
    UnsupportedOperation = -1020,
    InvalidTimestamp = -1021,
    InvalidSignature = -1022,
    StartTimeGreaterThanEndTime = -1023,
    NotFoundOrAllowed = -1099,

    // 11xx - 2xxx Request issues
    IllegalChars = -1100,
    TooManyParameters = -1101,
    MandatoryParameterEmptyOrMalformed = -1102,
    UnknownParameter = -1103,
    UnreadParameters = -1104,
    ParameterEmpty = -1105,
    ParameterNotRequired = -1106,
    BadAsset = -1108,
    BadAccount = -1109,
    BadInstrumentType = -1110,
    BadPrecision = -1111,
    NoDepth = -1112,
    WithdrawalNotNegative = -1113,
    TimeInForceNotRequired = -1114,
    InvalidTimeInForce = -1115,
    InvalidOrderType = -1116,
    InvalidSide = -1117,
    EmptyNewClientOrderId = -1118,
    EmptyOriginalClientOrderId = -1119,
    BadInterval = -1120,
    BadSymbol = -1121,
    InvalidListenKey = -1125,
    MoreThanXxHours = -1127,
    OptionalParametersBadCombination = -1128,
    InvalidParameter = -1130,
    BadReceiveWindow = -1131,

    // 20xx Processing issues
    BadApiId = -2008,
    NewOrderRejected = -2010,
    CancelRejected = -2011,
    NoSuchOrder = -2013,
    BadApiKeyFormat = -2014,
    RejectedApiKeyOrIp = -2015,
    NoTradingWindow = -2016,
    BalanceNotSufficient = -2018,
    MarginNotSufficient = -2019,
    UnableToFill = -2020,
    OrderWouldImmediatelyTrigger = -2021,
    ReduceOnlyReject = -2022,
    UserInLiquidation = -2023,
    PositionNotSufficient = -2024,
    MaxOpenOrderExceeded = -2025,
    ReduceOnlyOrderTypeNotSupported = -2026,
    MaxLeverageRatio = -2027,
    MinLeverageRatio = -2028,
}

impl Default for CommonCode {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for CommonCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as i16)
    }
}

#[derive(Clone, Copy, Debug, Deserialize_repr, FromPrimitive)]
#[repr(i16)]
pub enum FApiCode {
    Unknown = -1000,
    InvalidOrderStatus = -4000,
    PriceLessThanZero = -4001,
    PriceGreaterThanMaxPrice = -4002,
    QuantityLessThanZero = -4003,
    QuantityLessThanMinQuantity = -4004,
    QuantityLessThanMaxQuantity = -4005,
    StopPriceLessThanZero = -4006,
    StopPriceGreaterThanMaxPrice = -4007,
    TickSizeLessThanZero = -4008,
    MaxPriceLessThanMinPrice = -4009,
    MaxQuantityLessThanMinQuantity = -4010,
    StepSizeLessThanZero = -4011,
    MaxNumOfOrdersLessThanZero = -4012,
    PriceLessThanMinPrice = -4013,
    PriceNotIncreasedByTickSize = -4014,
    InvalidClientOrderIdLength = -4015,
    PriceHigherThanMultiplierUp = -4016,
    MultiplierUpLessThanZero = -4017,
    MultiplierDownLessThanZero = -4018,
    CompositeScaleOverflow = -4019,
    TargetStrategyInvalid = -4020,
    InvalidDepthLimit = -4021,
    WrongMarketStatus = -4022,
    QuantityNotIncreasedByStepSize = -4023,
    PriceLowerThanMultiplierDown = -4024,
    MultiplierDecimalLessThanZero = -4025,
    CommissionInvalid = -4026,
    InvalidAccountType = -4027,
    InvalidLeverage = -4028,
    InvalidTickSizePrecision = -4029,
    InvalidStepSizePrecision = -4030,
    InvalidWorkingType = -4031,
    ExceedMaxCancelOrderSize = -4032,
    InsuranceAccountNotFound = -4033,
    InvalidBalanceType = -4044,
    MaxStopOrderExceeded = -4045,
    NoNeedToChangeMarginType = -4046,
    ThereExistsOpenOrders = -4047,
    ThereExistsQuantity = -4048,
    AddIsolatedMarginReject = -4049,
    CrossBalanceInsufficient = -4050,
    IsolatedBalanceInsufficient = -4051,
    NoNeedToChangeAutoAddMargin = -4052,
    AutoAddCrossedMarginReject = -4053,
    AddIsolatedMarginNoPositionReject = -4054,
    AmountMustBePositive = -4055,
    InvalidApiKeyType = -4056,
    InvalidRsaPublicKey = -4057,
    MaxPriceTooLarge = -4058,
    NoNeedToChangePositionSide = -4059,
    InvalidPositionSide = -4060,
    PositionSideNotMatch = -4061,
    ReduceOnlyConflict = -4062,
    InvalidOptionsRequestType = -4063,
    InvalidOptionsTimeFrame = -4064,
    InvalidOptionsAmount = -4065,
    InvalidOptionsEventType = -4066,
    PositionSideChangeExistsOpenOrders = -4067,
    PositionSideChangeExistsQuantity = -4068,
    InvalidOptionsPremiumFee = -4069,
    InvalidClientOptionsIdLength = -4070,
    InvalidOptionsDirection = -4071,
    OptionsPremiumNotUpdate = -4072,
    OptionsPremiumInputLessThanZero = -4073,
    OptionsAmountBiggerThanUpper = -4074,
    OptionsPremiumOutputZero = -4075,
    OptionsPremiumTooDifferent = -4076,
    OptionsPremiumReachLimit = -4077,
    OptionsCommonError = -4078,
    InvalidOptionsId = -4079,
    OptionsUserNotFound = -4080,
    OptionsNotFound = -4081,
    InvalidBatchPlaceOrderSize = -4082,
    PlaceBatchOrdersFail = -4083,
    UpcomingMethod = -4084,
    InvalidNotionalLimitCoefficient = -4085,
    InvalidPriceSpreadThreshold = -4086,
}

impl Default for FApiCode {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for FApiCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as i16)
    }
}

#[derive(Clone, Copy, Debug, Deserialize_repr, FromPrimitive)]
#[repr(i16)]
pub enum SApiCode {
    Unknown = -1000,
    PairAdminBanTrade = -3021,
    AccountBanTrade = -3022,
    WarningMarginLevel = -3023,
    FewLiabilityLeft = -3024,
    InvalidEffectiveTime = -3025,
    ValidationFailed = -3026,
    NotValidMarginAsset = -3027,
    NotValidMarginPair = -3028,
    TransferFailed = -3029,
    AccountBanRepay = -3036,
    ProfitAndLossClearing = -3037,
    ListenKeyNotFound = -3038,
    PriceIndexNotFound = -3042,
    NotWhitelistUser = -3999,
    CapitalInvalid = -4001,
    CapitalInvalidGet = -4002,
    CapitalInvalidEmail = -4003,
    CapitalUnauthenticated = -4004,
    CapitalTooManyRequests = -4005,
    CapitalOnlySupportPrimaryAccount = -4006,
    CapitalAddressVerificationNotPass = -4007,
    CapitalAddressTagVerificationNotPass = -4008,
    AssetNotSupported = -5011,

    // 6xxx Savings issues
    DailyProductNotExists = -6001,
    DailyProductNotAccessible = -6003,
    DailyProductNotPurchasable = -6004,
    DailyLowerThanMinPurchaseLimit = -6005,
    DailyRedeemAmountError = -6006,
    DailyRedeemTimeError = -6007,
    DailyProductNotRedeemable = -6008,
    RequestFrequencyTooHigh = -6009,
    ExceededUserPurchaseLimit = -6011,
    BalanceNotEnough = -6012,
    PurchasingFailed = -6013,
    UpdateFailed = -6014,
    EmptyRequestBody = -6015,
    ParametersError = -6016,
    NotInWhitelist = -6017,
    AssetNotEnough = -6018,
    Pending = -6019,
}

impl Default for SApiCode {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for SApiCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as i16)
    }
}

#[derive(Clone, Copy, Debug, Deserialize_repr, FromPrimitive)]
#[repr(i16)]
pub enum WSApiCode {
    UnknownProperty = 0,
    InvalidValueType = 1,
    InvalidRequest = 2,
    InvalidJson = 3,
}

impl Default for WSApiCode {
    fn default() -> Self {
        Self::UnknownProperty
    }
}

impl fmt::Display for WSApiCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as i16)
    }
}

pub trait ApiCode:
    Clone + Copy + Default + fmt::Debug + fmt::Display + FromPrimitive + Send + Sync + 'static
{
}

impl ApiCode for FApiCode {}
impl ApiCode for SApiCode {}
impl ApiCode for WSApiCode {}

#[derive(Clone, Copy, Debug)]
pub enum Code<C: ApiCode> {
    Common(CommonCode),
    Api(C),
    Filter(i16),
}

impl<C> TryFrom<i16> for Code<C>
where
    C: ApiCode,
{
    type Error = &'static str;

    fn try_from(value: i16) -> result::Result<Self, Self::Error> {
        // Filter code range.
        if value <= -9000 {
            Ok(Self::Filter(value as i16))

        // API-specific code range.
        } else if value <= -3000 || value >= 0 {
            if let Some(code) = FromPrimitive::from_i16(value as i16) {
                Ok(Self::Api(code))
            } else {
                Err("a signed 16-bit integer")
            }

        // Common code range.
        } else {
            if let Some(code) = FromPrimitive::from_i16(value as i16) {
                Ok(Self::Common(code))
            } else {
                Err("a signed 16-bit integer")
            }
        }
    }
}

impl<C> Default for Code<C>
where
    C: ApiCode,
{
    fn default() -> Self {
        Self::Common(Default::default())
    }
}

impl<'de, C> Deserialize<'de> for Code<C>
where
    C: ApiCode,
{
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct CodeVisitor<C> {
            _phantom: PhantomData<C>,
        }

        impl<C> CodeVisitor<C> {
            fn new() -> Self {
                Self {
                    _phantom: PhantomData,
                }
            }
        }

        impl<'de, C> de::Visitor<'de> for CodeVisitor<C>
        where
            C: ApiCode,
        {
            type Value = Code<C>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a signed 16-bit integer")
            }

            fn visit_i64<E>(self, value: i64) -> result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                let err =
                    E::invalid_value(de::Unexpected::Signed(value), &"a signed 16-bit integer");

                // Value must be within signed 16-bit integer range.
                if value < i64::from(i16::MIN) || value > i64::from(i16::MAX) {
                    Err(err)
                } else {
                    Self::Value::try_from(value as i16).map_err(|_| err)
                }
            }
        }

        deserializer.deserialize_i16(CodeVisitor::<C>::new())
    }
}

impl<C> fmt::Display for Code<C>
where
    C: ApiCode,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Common(c) => fmt::Display::fmt(c, f),
            Self::Api(c) => fmt::Display::fmt(c, f),
            Self::Filter(c) => fmt::Display::fmt(c, f),
        }
    }
}

#[derive(Clone, Constructor, Debug, Default, Deserialize, thiserror::Error)]
#[error("({code}) {msg}")]
pub struct BinanceError<C: ApiCode> {
    code: Code<C>,
    msg: String,
}

impl<C> BinanceError<C>
where
    C: ApiCode,
{
    pub fn code(&self) -> Code<C> {
        self.code
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error<C: ApiCode> {
    #[error("API timeout")]
    ApiTimeout,

    #[error("Bad request: {0}")]
    BadRequest(#[source] BinanceError<C>),

    #[error("Firewall limit reached")]
    FirewallLimitReached,

    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("IP address has been banned")]
    IPAddressBanned,

    #[error("Request encoding error: {0}")]
    RequestEncoding(#[from] serde_urlencoded::ser::Error),

    #[error("Request rate limit reached")]
    RequestRateLimitReached,

    #[error("Response decoding error: {0}")]
    ResponseDecoding(#[from] serde_json::Error),

    #[error("Internal server error: {0}")]
    Server(#[source] BinanceError<C>),

    #[error("Websocket error: {0}")]
    Websocket(#[from] async_tungstenite::tungstenite::Error),

    #[error("Websocket is closed")]
    WebsocketClosed,

    #[error("Websocket request error: {0}")]
    WebsocketRequest(#[source] BinanceError<C>),

    #[error("Websocket request cancelled")]
    WebsocketRequestCancelled,

    #[error("Websocket request timed out")]
    WebsocketRequestTimeout,
}
