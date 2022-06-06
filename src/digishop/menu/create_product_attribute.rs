use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateProductAttributeBody {
    pub menu_type: String,
    pub identifier: String,
    pub name: Option<String>,
    pub attribute_category: Option<String>,
    pub characteristics: Option<Vec<ProductAttributeCharacteristicBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAttributeCharacteristicBody {
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProductAttributeResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateProductAttributeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for CreateProductAttributeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateProductAttributeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateProductAttributeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
