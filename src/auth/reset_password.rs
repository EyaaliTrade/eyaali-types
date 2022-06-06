use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct ResetPasswordBody {
    pub password_code: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetPasswordResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum ResetPasswordError {
    #[display(fmt = "error_hashing_password")]
    HashPassword,
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    Default(String),
}

impl ResponseError for ResetPasswordError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ResetPasswordError::HashPassword => {
                HttpResponse::InternalServerError().body("error_hashing_password")
            }
            ResetPasswordError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            ResetPasswordError::UserNotFound => HttpResponse::NotFound().body("user_not_found"),
            ResetPasswordError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
