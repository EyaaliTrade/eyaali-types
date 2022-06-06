use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckProductStockIsActiveBody {
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckProductStockIsActiveResult {
    pub stock_is_active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductStockIsActiveAggregation {
    pub stock_is_active: Option<bool>,
}

#[derive(Debug, Display)]
pub enum CheckProductStockIsActiveError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for CheckProductStockIsActiveError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CheckProductStockIsActiveError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CheckProductStockIsActiveError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
