use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RemoveFromMenuCartBody {
    pub device: String,
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveFromMenuCartResult {
    pub count: i32,
}

#[derive(Debug, Display)]
pub enum RemoveFromMenuCartError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for RemoveFromMenuCartError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RemoveFromMenuCartError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            RemoveFromMenuCartError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            RemoveFromMenuCartError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
