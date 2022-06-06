use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct BindUserOrderBody {
    pub user: String,
    pub device: String,
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BindUserOrderResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum BindUserOrderError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for BindUserOrderError {
    fn error_response(&self) -> HttpResponse {
        match self {
            BindUserOrderError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            BindUserOrderError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
