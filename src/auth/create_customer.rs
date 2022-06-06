use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateCustomerBody {
    pub menu: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: CustomerPhoneBody,
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
pub struct CreateCustomerResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateCustomerError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "error_hashing_password")]
    HashPassword,
    #[display(fmt = "email_already_exists")]
    EmailExists,
    #[display(fmt = "phone_already_exists")]
    PhoneExists,
    Default(String),
}

impl ResponseError for CreateCustomerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateCustomerError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateCustomerError::HashPassword => {
                HttpResponse::InternalServerError().body("error_hashing_password")
            }
            CreateCustomerError::EmailExists => HttpResponse::Conflict().body("email_already_exists"),
            CreateCustomerError::PhoneExists => HttpResponse::Conflict().body("phone_already_exists"),
            CreateCustomerError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
