use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SignUpCustomerBody {
    pub language_code: Option<String>,
    pub menu: String,
    #[validate(email)]
    pub email: String,
    pub username: Option<String>,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub phone: Option<CustomerPhoneBody>,
    pub is_verified: bool,
    pub company: Option<String>,
    pub kind: Option<String>,
    pub activation_status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerPhoneBody {
    pub country_code: i32,
    pub number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignUpCustomerResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum SignUpCustomerError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "error_hashing_password")]
    HashPassword,
    #[display(fmt = "email_already_exists")]
    EmailExists,
    #[display(fmt = "username_already_exists")]
    UsernameExists,
    #[display(fmt = "phone_already_exists")]
    PhoneExists,
    Default(String),
}

impl ResponseError for SignUpCustomerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SignUpCustomerError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SignUpCustomerError::HashPassword => {
                HttpResponse::InternalServerError().body("error_hashing_password")
            }
            SignUpCustomerError::EmailExists => HttpResponse::Conflict().body("email_already_exists"),
            SignUpCustomerError::UsernameExists => HttpResponse::Conflict().body("username_already_exists"),
            SignUpCustomerError::PhoneExists => HttpResponse::Conflict().body("phone_already_exists"),
            SignUpCustomerError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
