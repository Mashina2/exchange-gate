use std::collections::HashMap;

use serde::*;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum GateErr {
    #[error(transparent)]
    ReqwestHeaderError(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    BinanceContentError(#[from] BinanceContentError),
    #[error(transparent)]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    TonicError(#[from] tonic::transport::Error),

    #[error("Binance Server Error")]
    BinanceServerErr,
    #[error("binance unavailable")]
    BinanceUnavailable,
    #[error("binance unauthorized")]
    BinanceUnauthorized,
    #[error("`{0}`")]
    BinanceOtherErr(String),
    #[error("fail to get Timestamp")]
    GetTimestampErr,
    #[error("`{0}`")]
    CustomErr(String),
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[error("({:?}) {:?}\n{:?}", code, msg, extra)]
pub struct BinanceContentError {
    pub code: i16,
    pub msg: String,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
