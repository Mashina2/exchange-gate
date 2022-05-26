use crate::error::{BinanceContentError, GateErr};
use crate::ex_grpc::ex_gate::{Balance, BalancesReply};
use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use lazy_static::lazy_static;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::{Response, StatusCode};
use serde_json::Value;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
  /// This is an example for using doc comment attributes
  static ref BINANCE_CLIENT: Client = {
    let api_key = env::var("BINANCE_KEY").expect("BINANCE_KEY is not set in .env file");
    let secret_key = env::var("BINANCE_SECRET").expect("BINANCE_SECRET is not set in .env file");
    let host = env::var("BINANCE_HOST").expect("BINANCE_HOST is not set in .env file");
    Client::new(api_key, secret_key, host)
  };
}

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    host: String,
}

impl Client {
    pub fn new(api_key: String, secret_key: String, host: String) -> Self {
        Client {
            api_key,
            secret_key,
            host,
        }
    }

    fn sign_request(&self, endpoint: &str, request: &str) -> String {
        let mut signed_key = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        signed_key.update(request.as_bytes());
        let signature = hex_encode(signed_key.finalize().into_bytes());
        let request_body: String = format!("{}&signature={}", request, signature);
        let url: String = format!("{}{}?{}", self.host, endpoint, request_body);

        url
    }

    fn build_headers(&self, content_type: bool) -> Result<HeaderMap, GateErr> {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("ott"));
        if content_type {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );

        Ok(custom_headers)
    }

    async fn handler(&self, response: Response) -> Result<String, GateErr> {
        match response.status() {
            StatusCode::OK => Ok(response.text().await?),
            StatusCode::INTERNAL_SERVER_ERROR => Err(GateErr::BinanceServerErr),
            StatusCode::SERVICE_UNAVAILABLE => Err(GateErr::BinanceUnavailable),
            StatusCode::UNAUTHORIZED => Err(GateErr::BinanceUnauthorized),
            StatusCode::BAD_REQUEST => Err(GateErr::BinanceContentError(
                response.json::<BinanceContentError>().await?,
            )),
            s => Err(GateErr::BinanceOtherErr(format!(
                "Received response: {:?}",
                s
            ))),
        }
    }

    pub async fn get_signed(&self, endpoint: &str, request: &str) -> Result<String, GateErr> {
        let url = self.sign_request(endpoint, request);
        let client = reqwest::Client::new();
        let response = client
            .get(url.as_str())
            .headers(self.build_headers(true)?)
            .send()
            .await?;

        self.handler(response).await
    }
}

fn get_timestamp() -> Result<u64, GateErr> {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH)?;

    Ok(since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000)
}

pub fn build_signed_request(
    mut parameters: BTreeMap<String, String>,
    recv_window: u64,
) -> Result<String, GateErr> {
    if recv_window > 0 {
        parameters.insert("recvWindow".into(), recv_window.to_string());
    }

    if let Ok(timestamp) = get_timestamp() {
        parameters.insert("timestamp".into(), timestamp.to_string());

        let mut request = String::new();
        for (key, value) in &parameters {
            let param = format!("{}={}&", key, value);
            request.push_str(param.as_ref());
        }
        request.pop(); // remove last &

        Ok(request)
    } else {
        Err(GateErr::GetTimestampErr)
    }
}

pub async fn get_account() -> Result<BalancesReply, GateErr> {
    let parameters: BTreeMap<String, String> = BTreeMap::new();

    let request = build_signed_request(parameters, 5000)?;
    let data = BINANCE_CLIENT
        .get_signed("/api/v3/account", &request)
        .await?;
    let account_info: Value = serde_json::from_str(data.as_str())?;
    let balances = account_info["balances"]
        .as_array()
        .ok_or(GateErr::CustomErr("binance no balance field".to_string()))?;
    let balances = balances
        .into_iter()
        .map(|token| Balance {
            asset: token["asset"].to_string(),
            free: token["free"].to_string(),
            locked: token["locked"].to_string(),
        })
        .collect();
    let balances: BalancesReply = BalancesReply { balances: balances };
    Ok(balances)
}
