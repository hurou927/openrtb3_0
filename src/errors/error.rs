use actix_web::{error, Result};
use derive_more::{Display, Error};
use serde_json;
use std::fmt;

#[derive(Debug, Display, Error)]
#[display(fmt = "service error. type: {} ,detail: {}", code, detail)]
pub struct ServiceError {
    pub code: ErrorCode,
    pub detail: String,
}

#[derive(Debug)]
pub enum ErrorCode {
    RequestError,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorCode::RequestError => write!(f, "RequestError"),
        }
    }
}
