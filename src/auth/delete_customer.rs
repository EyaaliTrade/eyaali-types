use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DeleteCustomerBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteCustomerResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteCustomerError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "action_not_allowed")]
    ActionNotAllowed,
    Default(String),
}

impl ResponseError for DeleteCustomerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteCustomerError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteCustomerError::ActionNotAllowed => {
                HttpResponse::Forbidden().body("action_not_allowed")
            }
            DeleteCustomerError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
