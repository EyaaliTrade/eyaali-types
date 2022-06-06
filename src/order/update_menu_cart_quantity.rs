use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuCartQuantityBody {
    pub device: String,
    pub product: String,
    pub id: Option<String>,
    pub quantity: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuCartQuantityResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuCartQuantityError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for UpdateMenuCartQuantityError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuCartQuantityError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuCartQuantityError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            UpdateMenuCartQuantityError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
