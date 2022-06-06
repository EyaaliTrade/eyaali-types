use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AddProductInventoryStockBody {
    pub product: String,
    pub variant: Option<String>,
    pub site: Option<String>,
    pub date: DateTime<Utc>,
    pub cost: Option<f64>,
    pub quantity: f64,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddProductInventoryStockResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum AddProductInventoryStockError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for AddProductInventoryStockError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AddProductInventoryStockError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            AddProductInventoryStockError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            AddProductInventoryStockError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            AddProductInventoryStockError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
