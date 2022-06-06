use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetProductOptionsByItemBody {
    pub item: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductOptionsByItemResult {
    pub product: Option<String>,
    pub options: Option<Vec<OptionValueAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductOptionsByItemAggregation {
    pub product: Option<String>,
    pub options: Option<Vec<OptionValueAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionValueAggregation {
    pub option: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetProductOptionsByItemError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductOptionsByItemError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductOptionsByItemError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductOptionsByItemError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
