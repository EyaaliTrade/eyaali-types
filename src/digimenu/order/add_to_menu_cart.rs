use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AddToMenuCartBody {
    pub menu: String,
    pub device: String,
    pub product: String,
    pub quantity: f64,
    pub unit_price: PriceBody,
    pub supplements: Option<Vec<MenuCartSupplementBody>>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
    pub product_is_available: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceBody {
    pub value: f64,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCartSupplementBody {
    pub id: Option<String>,
    pub quantity: Option<f64>,
    pub unit_price: Option<PriceBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddToMenuCartResult {
    pub count: i32,
}

#[derive(Debug, Display)]
pub enum AddToMenuCartError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_product")]
    InvalidProduct,
    #[display(fmt = "product_already_added")]
    ProductAlreadyAdded,
    #[display(fmt = "product_not_yet_available")]
    ProductNotYetAvailable,
    Default(String),
}

impl ResponseError for AddToMenuCartError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AddToMenuCartError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            AddToMenuCartError::InvalidProduct => HttpResponse::Conflict().body("invalid_product"),
            AddToMenuCartError::ProductAlreadyAdded => {
                HttpResponse::Gone().body("product_already_added")
            }
            AddToMenuCartError::ProductNotYetAvailable => HttpResponse::Conflict().body("product_not_yet_available"),
            AddToMenuCartError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
