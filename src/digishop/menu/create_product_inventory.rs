use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateProductInventoryBody {
    pub product: String,
    pub variant: Option<String>,
    pub kind: Option<String>,
    pub price: Option<ProductInventoryPriceBody>,
    pub quantity: Option<f64>,
    pub barcode: Option<String>,
    pub sku: Option<String>,
    pub acquisition_date: Option<DateTime<Utc>>,
    pub sites: Option<Vec<ProductInventorySiteBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductInventoryPriceBody {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductInventorySiteBody {
    pub site: Option<String>,
    pub quantity: Option<f64>,
    pub sku: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProductInventoryResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateProductInventoryError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for CreateProductInventoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateProductInventoryError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateProductInventoryError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            CreateProductInventoryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
