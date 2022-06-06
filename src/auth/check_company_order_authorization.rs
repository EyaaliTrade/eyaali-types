use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CheckCompanyOrderAuthorizationBody {
    pub company: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckCompanyOrderAuthorizationResult {
    pub is_authorized: bool,
}

#[derive(Debug, Display)]
pub enum CheckCompanyOrderAuthorizationError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for CheckCompanyOrderAuthorizationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CheckCompanyOrderAuthorizationError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CheckCompanyOrderAuthorizationError::Default(error) => {
                HttpResponse::BadRequest().body(error)
            }
        }
    }
}
