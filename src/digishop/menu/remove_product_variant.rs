use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RemoveProductVariantBody {
    pub product: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveProductVariantResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum RemoveProductVariantError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    #[display(fmt = "variant_not_found")]
    VariantNotFound,
    Default(String),
}

impl ResponseError for RemoveProductVariantError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RemoveProductVariantError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            RemoveProductVariantError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            RemoveProductVariantError::VariantNotFound => {
                HttpResponse::Conflict().body("variant_not_found")
            }
            RemoveProductVariantError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
