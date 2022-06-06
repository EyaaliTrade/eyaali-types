use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuOrderStatusBody {
    pub order: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuOrderStatusResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuOrderStatusError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "order_not_found")]
    OrderNotFound,
    Default(String),
}

impl ResponseError for UpdateMenuOrderStatusError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuOrderStatusError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuOrderStatusError::OrderNotFound => {
                HttpResponse::Conflict().body("order_not_found")
            }
            UpdateMenuOrderStatusError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
