use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuCartQuantityBody {
    pub user: Option<String>,
    pub device: Option<String>,
    pub item: String,
    pub quantity: f64,
    pub product_is_available: bool,
    pub quantity_is_sufficient: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuCartQuantityResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuCartQuantityError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    #[display(fmt = "item_not_found")]
    ItemNotFound,
    #[display(fmt = "product_not_yet_available")]
    ProductNotYetAvailable,
    #[display(fmt = "insufficient_quantity")]
    InsufficientQuantity,
    Default(String),
}

impl ResponseError for UpdateMenuCartQuantityError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuCartQuantityError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuCartQuantityError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            UpdateMenuCartQuantityError::ItemNotFound => {
                HttpResponse::Conflict().body("item_not_found")
            }
            UpdateMenuCartQuantityError::ProductNotYetAvailable => HttpResponse::Conflict().body("product_not_yet_available"),
            UpdateMenuCartQuantityError::InsufficientQuantity => HttpResponse::Conflict().body("insufficient_quantity"),
            UpdateMenuCartQuantityError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
