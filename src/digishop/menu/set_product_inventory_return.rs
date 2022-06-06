use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SetProductInventoryReturnBody {
    pub order: String,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetProductInventoryReturnResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum SetProductInventoryReturnError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "inventory_not_found")]
    InventoryNotFound,
    Default(String),
}

impl ResponseError for SetProductInventoryReturnError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SetProductInventoryReturnError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SetProductInventoryReturnError::InventoryNotFound => {
                HttpResponse::Conflict().body("inventory_not_found")
            }
            SetProductInventoryReturnError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
