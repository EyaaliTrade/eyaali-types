use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuOrderQuantityBody {
    pub order: String,
    pub item: String,
    pub quantity: f64,
    pub product_is_available: bool,
    pub quantity_is_sufficient: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuOrderQuantityResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuOrderQuantityError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    #[display(fmt = "action_not_allowed")]
    ActionNotAllowed,
    #[display(fmt = "item_not_found")]
    ItemNotFound,
    #[display(fmt = "product_not_yet_available")]
    ProductNotYetAvailable,
    #[display(fmt = "insufficient_quantity")]
    InsufficientQuantity,
    Default(String),
}

impl ResponseError for UpdateMenuOrderQuantityError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuOrderQuantityError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuOrderQuantityError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            UpdateMenuOrderQuantityError::ActionNotAllowed => {
                HttpResponse::Forbidden().body("action_not_allowed")
            }
            UpdateMenuOrderQuantityError::ItemNotFound => {
                HttpResponse::Conflict().body("item_not_found")
            }
            UpdateMenuOrderQuantityError::ProductNotYetAvailable => HttpResponse::Conflict().body("product_not_yet_available"),
            UpdateMenuOrderQuantityError::InsufficientQuantity => HttpResponse::Conflict().body("insufficient_quantity"),
            UpdateMenuOrderQuantityError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
