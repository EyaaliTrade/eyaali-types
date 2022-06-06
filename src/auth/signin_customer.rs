use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignInCustomerBody {
    pub menu: String,
    pub login: String,
    pub password: String,
    pub fcm_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignInCustomerResult {
    pub id: String,
    pub token: String,
}

#[derive(Debug, Display)]
pub enum SignInCustomerError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "error_hashing_password")]
    HashPassword,
    #[display(fmt = "invalid_password")]
    InvalidPassword,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    #[display(fmt = "user_not_verified")]
    UserNotVerified,
    #[display(fmt = "inactif_user")]
    InActifUser,
    Default(String),
}

impl ResponseError for SignInCustomerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SignInCustomerError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SignInCustomerError::HashPassword => {
                HttpResponse::InternalServerError().body("error_hashing_password")
            }
            SignInCustomerError::InvalidPassword => {
                HttpResponse::Unauthorized().body("invalid_password")
            }
            SignInCustomerError::UserNotFound => HttpResponse::Conflict().body("user_not_found"),
            SignInCustomerError::UserNotVerified => HttpResponse::Gone().body("user_not_verified"),
            SignInCustomerError::InActifUser => HttpResponse::Forbidden().body("inactif_user"),
            SignInCustomerError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
