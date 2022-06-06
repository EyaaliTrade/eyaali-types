use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct ResendCustomerActivationEmailBody {
    pub language_code: Option<String>,
    pub menu: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResendCustomerActivationEmailResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum ResendCustomerActivationEmailError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    #[display(fmt = "account_already_verified")]
    AccountAlreadyVerified,
    Default(String),
}

impl ResponseError for ResendCustomerActivationEmailError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ResendCustomerActivationEmailError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            ResendCustomerActivationEmailError::UserNotFound => HttpResponse::NotFound().body("user_not_found"),
            ResendCustomerActivationEmailError::AccountAlreadyVerified => HttpResponse::NotFound().body("account_already_verified"),
            ResendCustomerActivationEmailError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
