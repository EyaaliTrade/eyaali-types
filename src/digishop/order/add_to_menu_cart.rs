use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AddToMenuCartBody {
    pub user: Option<String>,
    pub device: Option<String>,
    pub menu: String,
    pub site: Option<String>,
    pub product: String,
    pub options: Option<Vec<MenuCartOptionBody>>,
    pub quantity: f64,
    pub unit_price: PriceBody,
    pub notes: Option<String>,
    pub product_is_available: bool,
    pub quantity_is_sufficient: bool,
    pub variant_is_exists: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCartOptionBody {
    pub option: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceBody {
    pub value: f64,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddToMenuCartResult {
    pub count: i32,
}

#[derive(Debug, Display)]
pub enum AddToMenuCartError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    #[display(fmt = "invalid_product")]
    InvalidProduct,
    #[display(fmt = "product_not_yet_available")]
    ProductNotYetAvailable,
    #[display(fmt = "insufficient_quantity")]
    InsufficientQuantity,
    #[display(fmt = "variant_does_not_exist")]
    VariantDoesNotExist,
    Default(String),
}

impl ResponseError for AddToMenuCartError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AddToMenuCartError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            AddToMenuCartError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            AddToMenuCartError::InvalidProduct => HttpResponse::Conflict().body("invalid_product"),
            AddToMenuCartError::ProductNotYetAvailable => HttpResponse::Conflict().body("product_not_yet_available"),
            AddToMenuCartError::InsufficientQuantity => HttpResponse::Conflict().body("insufficient_quantity"),
            AddToMenuCartError::VariantDoesNotExist => HttpResponse::Conflict().body("variant_does_not_exist"),
            AddToMenuCartError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
