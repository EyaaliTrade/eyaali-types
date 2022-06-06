use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RemoveProductInventoryBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveProductInventoryResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum RemoveProductInventoryError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "action_not_allowed")]
    ActionNotAllowed,
    #[display(fmt = "inventory_not_found")]
    InventoryNotFound,
    Default(String),
}

impl ResponseError for RemoveProductInventoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RemoveProductInventoryError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            RemoveProductInventoryError::ActionNotAllowed => {
                HttpResponse::Forbidden().body("action_not_allowed")
            }
            RemoveProductInventoryError::InventoryNotFound => {
                HttpResponse::Conflict().body("inventory_not_found")
            }
            RemoveProductInventoryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
