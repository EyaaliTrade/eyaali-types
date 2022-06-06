use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductVariantPricesBody {
    pub product: String,
    pub options: Vec<OptionValueBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionValueBody {
    pub option: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductVariantPricesResult {
    pub price_difference: Option<f64>,
    pub discount_percentage: Option<i32>,
}

#[derive(Debug, Display)]
pub enum GetProductVariantPricesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductVariantPricesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductVariantPricesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductVariantPricesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
