use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductAvailableQuantityBody {
    pub product: String,
    pub site: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductAvailableQuantityResult {
    pub available_quantity: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAvailableQuantityAggregation {
    pub available_quantity: Option<f64>,
}

#[derive(Debug, Display)]
pub enum GetProductAvailableQuantityError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductAvailableQuantityError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductAvailableQuantityError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductAvailableQuantityError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
