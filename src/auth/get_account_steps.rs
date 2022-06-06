use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountStepsBody {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountStepsResult {
    pub step: i32,
}

#[derive(Debug, Display)]
pub enum GetAccountStepsError {
    #[display(fmt = "user_not_found")]
    UserNotFound,
    Default(String),
}

impl ResponseError for GetAccountStepsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAccountStepsError::UserNotFound => HttpResponse::NotFound().body("user_not_found"),
            GetAccountStepsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
