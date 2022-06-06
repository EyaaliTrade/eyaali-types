use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductAvailableStockBody {
    pub product: String,
    pub site: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionValueBody {
    pub option: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductAvailableStockResult {
    pub list: Vec<ProductAvailableStockAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAvailableStockAggregation {
    pub id: Option<String>,
    pub available_quantity: Option<f64>,
}

#[derive(Debug, Display)]
pub enum GetProductAvailableStockError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductAvailableStockError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductAvailableStockError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductAvailableStockError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
