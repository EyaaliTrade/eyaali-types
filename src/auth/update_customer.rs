use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateCustomerBody {
    pub menu: String,
    pub id: String,
    #[validate(email)]
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<CustomerPhoneBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerPhoneBody {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCustomerResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateCustomerError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "action_not_allowed")]
    ActionNotAllowed,
    #[display(fmt = "customer_not_found")]
    CustomerNotFound,
    #[display(fmt = "email_already_exists")]
    EmailExists,
    #[display(fmt = "phone_already_exists")]
    PhoneExists,
    Default(String),
}

impl ResponseError for UpdateCustomerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateCustomerError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateCustomerError::ActionNotAllowed => {
                HttpResponse::Forbidden().body("action_not_allowed")
            }
            UpdateCustomerError::CustomerNotFound => HttpResponse::Conflict().body("customer_not_found"),
            UpdateCustomerError::EmailExists => HttpResponse::Conflict().body("email_already_exists"),
            UpdateCustomerError::PhoneExists => HttpResponse::Conflict().body("phone_already_exists"),
            UpdateCustomerError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
