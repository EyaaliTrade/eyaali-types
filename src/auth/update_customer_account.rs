use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateCustomerAccountBody {
    pub menu: String,
    pub id: String,
    #[validate(email)]
    pub email: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub picture: Option<String>,
    pub phone: Option<CustomerPhoneBody>,
    pub is_notified: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerPhoneBody {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCustomerAccountResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateCustomerAccountError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    #[display(fmt = "email_already_exists")]
    EmailExists,
    #[display(fmt = "username_already_exists")]
    UsernameExists,
    #[display(fmt = "phone_already_exists")]
    PhoneExists,
    Default(String),
}

impl ResponseError for UpdateCustomerAccountError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateCustomerAccountError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateCustomerAccountError::UserNotFound => HttpResponse::Conflict().body("user_not_found"),
            UpdateCustomerAccountError::EmailExists => HttpResponse::Conflict().body("email_already_exists"),
            UpdateCustomerAccountError::UsernameExists => HttpResponse::Conflict().body("username_already_exists"),
            UpdateCustomerAccountError::PhoneExists => HttpResponse::Conflict().body("phone_already_exists"),
            UpdateCustomerAccountError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
