use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DisableUserDeviceBody {
    pub user: String,
    pub device: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisableUserDeviceResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DisableUserDeviceError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DisableUserDeviceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DisableUserDeviceError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DisableUserDeviceError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
