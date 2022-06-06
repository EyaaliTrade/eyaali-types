use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AddProductInventoryLossBody {
    pub id: String,
    pub date: DateTime<Utc>,
    pub quantity: f64,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddProductInventoryLossResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum AddProductInventoryLossError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    #[display(fmt = "inventory_not_found")]
    InventoryNotFound,
    Default(String),
}

impl ResponseError for AddProductInventoryLossError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AddProductInventoryLossError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            AddProductInventoryLossError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            AddProductInventoryLossError::InventoryNotFound => {
                HttpResponse::Conflict().body("inventory_not_found")
            }
            AddProductInventoryLossError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
