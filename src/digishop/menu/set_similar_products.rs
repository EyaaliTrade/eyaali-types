use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSimilarProductsBody {
    pub product: String,
    pub is_random: bool,
    pub count: i32,
    pub category: Option<String>,
    pub products: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetSimilarProductsResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum SetSimilarProductsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for SetSimilarProductsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SetSimilarProductsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SetSimilarProductsError::ProductNotFound => {
                HttpResponse::NotFound().body("product_not_found")
            }
            SetSimilarProductsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
