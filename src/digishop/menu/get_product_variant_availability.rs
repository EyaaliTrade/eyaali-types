use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductVariantAvailabilityBody {
    pub product: String,
    pub variant: String,
    pub site: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductVariantAvailabilityResult {
    pub available_quantity: Option<f64>,
}

#[derive(Debug, Display)]
pub enum GetProductVariantAvailabilityError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductVariantAvailabilityError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductVariantAvailabilityError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductVariantAvailabilityError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
