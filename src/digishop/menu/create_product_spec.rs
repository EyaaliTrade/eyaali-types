use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateProductSpecBody {
    pub product: String,
    pub identifier: String,
    pub title: String,
    pub specifications: Option<Vec<ProductSpecificationBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductSpecificationBody {
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProductSpecResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateProductSpecError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for CreateProductSpecError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateProductSpecError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateProductSpecError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
