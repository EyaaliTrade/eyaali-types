use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateProductSpecBody {
    pub id: String,
    pub title: Option<String>,
    pub specifications: Option<Vec<ProductSpecificationBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductSpecificationBody {
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductSpecResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateProductSpecError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_spec_not_found")]
    ProductSpecNotFound,
    Default(String),
}

impl ResponseError for UpdateProductSpecError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateProductSpecError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateProductSpecError::ProductSpecNotFound => {
                HttpResponse::Conflict().body("product_spec_not_found")
            }
            UpdateProductSpecError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
