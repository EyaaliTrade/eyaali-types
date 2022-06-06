use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct ForgotCustomerPasswordBody {
    pub language_code: Option<String>,
    pub menu: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgotCustomerPasswordResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum ForgotCustomerPasswordError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    Default(String),
}

impl ResponseError for ForgotCustomerPasswordError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ForgotCustomerPasswordError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            ForgotCustomerPasswordError::UserNotFound => HttpResponse::NotFound().body("user_not_found"),
            ForgotCustomerPasswordError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
