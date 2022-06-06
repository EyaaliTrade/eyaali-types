use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct ChangePasswordBody {
    pub id: String,
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangePasswordResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum ChangePasswordError {
    #[display(fmt = "error_hashing_password")]
    HashPassword,
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_password")]
    InvalidPassword,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    Default(String),
}

impl ResponseError for ChangePasswordError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ChangePasswordError::HashPassword => {
                HttpResponse::InternalServerError().body("error_hashing_password")
            }
            ChangePasswordError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            ChangePasswordError::InvalidPassword => HttpResponse::Gone().body("invalid_password"),
            ChangePasswordError::UserNotFound => HttpResponse::Conflict().body("user_not_found"),
            ChangePasswordError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
