use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductInventoryInfoBody {
    pub product: String,
    pub variant: Option<String>,
    pub site: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductInventoryInfoResult {
    pub price: Option<ProductPriceAggregation>,
    pub barcode: Option<String>,
    pub sku: Option<String>,
    pub available_quantity: Option<f64>,
    pub items_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductInventoryInfoAggregation {
    pub price: Option<ProductPriceAggregation>,
    pub barcode: Option<String>,
    pub sku: Option<String>,
    pub available_quantity: Option<f64>,
    pub items_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPriceAggregation {
    pub value: Option<f64>,
    pub currency: Option<PriceCurrencyAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceCurrencyAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetProductInventoryInfoError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductInventoryInfoError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductInventoryInfoError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductInventoryInfoError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
