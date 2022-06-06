use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CheckMenuOrderBody {
    pub user: String,
    pub menu: String,
    pub delivery_method: String,
    pub delivery_address: Option<String>,
    pub payment_method: Option<String>,
    pub order_is_valid: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckMenuOrderResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CheckMenuOrderError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    #[display(fmt = "cart_is_empty")]
    CartIsEmpty,
    #[display(fmt = "invalid_order")]
    InvalidOrder,
    Default(String),
}

impl ResponseError for CheckMenuOrderError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CheckMenuOrderError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CheckMenuOrderError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            CheckMenuOrderError::CartIsEmpty => HttpResponse::Conflict().body("cart_is_empty"),
            CheckMenuOrderError::InvalidOrder => HttpResponse::Conflict().body("invalid_order"),
            CheckMenuOrderError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
