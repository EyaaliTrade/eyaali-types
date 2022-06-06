use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct ClearMenuCartBody {
    pub menu: String,
    pub device: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearMenuCartResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum ClearMenuCartError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "cart_already_cleared")]
    CartAlreadyCleared,
    Default(String),
}

impl ResponseError for ClearMenuCartError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ClearMenuCartError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            ClearMenuCartError::CartAlreadyCleared => {
                HttpResponse::Gone().body("cart_already_cleared")
            }
            ClearMenuCartError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
