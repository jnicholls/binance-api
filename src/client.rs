use std::marker::PhantomData;

use derive_more::Constructor;
use hmac::{Hmac, Mac, NewMac};
use serde::{de::DeserializeOwned, Serialize};
use sha2::Sha256;

use crate::error::*;

const SAPI_HOST: &str = "https://api.binance.com";
const FAPI_HOST: &str = "https://fapi.binance.com";

macro_rules! http_verb {
    ($method:ident) => {
        #[allow(dead_code)]
        pub(crate) async fn $method<I, O>(&self, path: &str, data: I) -> Result<O, A::ErrorCode>
        where
            I: Serialize,
            O: DeserializeOwned,
        {
            let url = self.prepare_url(path, data)?;
            let req = self.add_api_key(self.http.$method(&url));

            self.send_request(req).await
        }
    };
}

pub type FClient = Client<FApi>;
pub type SClient = Client<SApi>;

#[derive(Clone, Constructor, Debug)]
pub struct Credentials {
    api_key: String,
    secret_key: String,
}

impl Credentials {
    fn api_key(&self) -> &str {
        &self.api_key
    }

    fn sign(&self, data: impl AsRef<[u8]>) -> String {
        let mut hmac = Hmac::<Sha256>::new_varkey(self.secret_key.as_bytes())
            .expect("secret_key should be of the correct byte length");
        hmac.update(data.as_ref());
        hex::encode(hmac.finalize().into_bytes())
    }
}

#[derive(Clone, Debug)]
pub struct Client<A: Api> {
    creds: Option<Credentials>,
    http: reqwest::Client,
    _marker: PhantomData<A>,
}

impl<A> Client<A>
where
    A: Api,
{
    pub fn new() -> Self {
        Self {
            creds: None,
            http: reqwest::Client::new(),
            _marker: PhantomData,
        }
    }

    pub fn with_credentials(creds: Credentials) -> Self {
        let creds = Some(creds);

        Self {
            creds,
            http: reqwest::Client::new(),
            _marker: PhantomData,
        }
    }

    fn add_api_key(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(keys) = &self.creds {
            builder.header("X-MBX-APIKEY", keys.api_key())
        } else {
            builder
        }
    }

    fn prepare_url<I>(&self, path: &str, data: I) -> Result<String, A::ErrorCode>
    where
        I: Serialize,
    {
        let mut query = serde_urlencoded::to_string(data)?;

        if let Some(keys) = &self.creds {
            query = format!(
                "{}&timestamp={}",
                query,
                chrono::Utc::now().timestamp_millis()
            );
            query = format!("{}&signature={}", query, keys.sign(&query));
        }

        Ok(format!("{}{}?{}", A::host(), path, query))
    }

    async fn send_request<O>(&self, req: reqwest::RequestBuilder) -> Result<O, A::ErrorCode>
    where
        O: DeserializeOwned,
    {
        let resp = req.send().await?;

        match resp.status().as_u16() {
            200 => Ok(resp.json().await?),
            403 => Err(Error::FirewallLimitReached),
            418 => Err(Error::IPAddressBanned),
            429 => Err(Error::RequestRateLimitReached),
            400..=499 => Err(Error::BadRequest(resp.json().await?)),
            503 => Err(Error::ApiTimeout),
            500..=599 => Err(Error::Server(resp.json().await?)),
            _ => Ok(resp.error_for_status()?.json().await?),
        }
    }

    http_verb!(delete);
    http_verb!(get);
    http_verb!(patch);
    http_verb!(post);
    http_verb!(put);
}

pub trait Api: Clone + Send + Sync {
    type ErrorCode: ApiCode + DeserializeOwned;

    fn host() -> &'static str;
}

#[derive(Clone, Debug)]
pub struct FApi;
impl Api for FApi {
    type ErrorCode = FApiCode;

    fn host() -> &'static str {
        FAPI_HOST
    }
}

#[derive(Clone, Debug)]
pub struct SApi;
impl Api for SApi {
    type ErrorCode = SApiCode;

    fn host() -> &'static str {
        SAPI_HOST
    }
}
