use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CheckMenuOrderBody {
    pub menu: String,
    pub device: String,
    pub payment_method: Option<String>,
    pub customer_data: CustomerDataBody,
    pub payment_is_valid: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerDataBody {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: CustomerPhoneBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerPhoneBody {
    pub country_code: i32,
    pub number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckMenuOrderResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CheckMenuOrderError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "cart_is_empty")]
    CartIsEmpty,
    #[display(fmt = "payment_order")]
    InvalidPayment,
    Default(String),
}

impl ResponseError for CheckMenuOrderError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CheckMenuOrderError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CheckMenuOrderError::CartIsEmpty => HttpResponse::Conflict().body("cart_is_empty"),
            CheckMenuOrderError::InvalidPayment => HttpResponse::Conflict().body("invalid_payment"),
            CheckMenuOrderError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
