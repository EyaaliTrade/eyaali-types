use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateOrderPaymentStatusBody {
    pub order: String,
    pub payment_status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOrderPaymentStatusResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateOrderPaymentStatusError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "order_not_found")]
    OrderNotFound,
    Default(String),
}

impl ResponseError for UpdateOrderPaymentStatusError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateOrderPaymentStatusError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateOrderPaymentStatusError::OrderNotFound => {
                HttpResponse::Conflict().body("order_not_found")
            }
            UpdateOrderPaymentStatusError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
