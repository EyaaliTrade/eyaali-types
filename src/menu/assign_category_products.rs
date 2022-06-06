use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AssignCategoryProductsBody {
    pub category: String,
    pub products: Vec<ProductIdBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductIdBody {
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssignCategoryProductsResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum AssignCategoryProductsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for AssignCategoryProductsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AssignCategoryProductsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            AssignCategoryProductsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
