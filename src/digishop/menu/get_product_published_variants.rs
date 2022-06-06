use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductPublishedVariantsBody {
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductPublishedVariantsResult {
    pub list: Vec<ProductVariantAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductVariantAggregation {
    pub id: Option<String>,
    pub options: Option<Vec<ProductVariantOptionAggregation>>,
    pub names: Option<Vec<ProductVariantNameAggregation>>,
    pub picture: Option<ProductPictureUrlAggregation>,
    pub price_difference: Option<f64>,
    pub discount_percentage: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductVariantOptionAggregation {
    pub option: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductVariantNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetProductPublishedVariantsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductPublishedVariantsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductPublishedVariantsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductPublishedVariantsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
