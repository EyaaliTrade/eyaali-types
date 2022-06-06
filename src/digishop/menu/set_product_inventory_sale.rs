use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SetProductInventorySaleBody {
    pub id: String,
    pub order: String,
    pub date: DateTime<Utc>,
    pub quantity: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetProductInventorySaleResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum SetProductInventorySaleError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "inventory_not_found")]
    InventoryNotFound,
    Default(String),
}

impl ResponseError for SetProductInventorySaleError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SetProductInventorySaleError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SetProductInventorySaleError::InventoryNotFound => {
                HttpResponse::Conflict().body("inventory_not_found")
            }
            SetProductInventorySaleError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
