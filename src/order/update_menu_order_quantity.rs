use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuOrderQuantityBody {
    pub order: String,
    pub product: String,
    pub quantity: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuOrderQuantityResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuOrderQuantityError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "action_not_allowed")]
    ActionNotAllowed,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for UpdateMenuOrderQuantityError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuOrderQuantityError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuOrderQuantityError::ActionNotAllowed => {
                HttpResponse::Forbidden().body("action_not_allowed")
            }
            UpdateMenuOrderQuantityError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            UpdateMenuOrderQuantityError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
