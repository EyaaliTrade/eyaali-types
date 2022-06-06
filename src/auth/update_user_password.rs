use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateUserPasswordBody {
    pub id: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserPasswordResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateUserPasswordError {
    #[display(fmt = "error_hashing_password")]
    HashPassword,
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    Default(String),
}

impl ResponseError for UpdateUserPasswordError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateUserPasswordError::HashPassword => {
                HttpResponse::InternalServerError().body("error_hashing_password")
            }
            UpdateUserPasswordError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateUserPasswordError::UserNotFound => {
                HttpResponse::Conflict().body("user_not_found")
            }
            UpdateUserPasswordError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
