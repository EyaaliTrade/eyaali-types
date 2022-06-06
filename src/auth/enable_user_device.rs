use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct EnableUserDeviceBody {
    pub user: String,
    pub device: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnableUserDeviceResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum EnableUserDeviceError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for EnableUserDeviceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            EnableUserDeviceError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            EnableUserDeviceError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
