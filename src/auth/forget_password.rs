use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct ForgetPasswordBody {
    pub sender: String,
    pub language_code: Option<String>,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgetPasswordResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum ForgetPasswordError {
    #[display(fmt = "user_not_found")]
    UserNotFound,
    Default(String),
}

impl ResponseError for ForgetPasswordError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ForgetPasswordError::UserNotFound => HttpResponse::NotFound().body("user_not_found"),
            ForgetPasswordError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
