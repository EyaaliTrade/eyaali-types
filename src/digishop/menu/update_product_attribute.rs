use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateProductAttributeBody {
    pub id: String,
    pub name: Option<UpdateProductAttributeNameBody>,
    pub attribute_category: Option<String>,
    pub characteristics: Option<Vec<ProductAttributeCharacteristicBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductAttributeNameBody {
    pub id: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAttributeCharacteristicBody {
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductAttributeResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateProductAttributeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_attribute_not_found")]
    ProductAttributeNotFound,
    Default(String),
}

impl ResponseError for UpdateProductAttributeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateProductAttributeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateProductAttributeError::ProductAttributeNotFound => {
                HttpResponse::Conflict().body("product_attribute_not_found")
            }
            UpdateProductAttributeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
