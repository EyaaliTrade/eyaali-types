use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RemoveFromMenuOrderBody {
    pub order: String,
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveFromMenuOrderResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum RemoveFromMenuOrderError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "action_not_allowed")]
    ActionNotAllowed,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for RemoveFromMenuOrderError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RemoveFromMenuOrderError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            RemoveFromMenuOrderError::ActionNotAllowed => {
                HttpResponse::Forbidden().body("action_not_allowed")
            }
            RemoveFromMenuOrderError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            RemoveFromMenuOrderError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
