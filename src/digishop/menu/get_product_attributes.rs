use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductAttributesBody {
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductAttributesResult {
    pub list: Vec<ProductAttributeAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAttributeAggregation {
    pub id: Option<String>,
    pub name: Option<ProductAttributeNameAggregation>,
    pub attribute_category: Option<AttributeCategoryAggregation>,
    pub characteristics: Option<Vec<ProductAttributeCharacteristicAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAttributeNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeCategoryAggregation {
    pub id: Option<String>,
    pub name: Option<ProductAttributeNameAggregation>,
    pub picture: Option<PictureUrlAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAttributeCharacteristicAggregation {
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetProductAttributesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductAttributesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductAttributesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductAttributesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
