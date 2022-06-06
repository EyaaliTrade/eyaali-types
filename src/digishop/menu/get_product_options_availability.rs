use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductOptionsAvailabilityBody {
    pub product: String,
    pub site: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductOptionsAvailabilityResult {
    pub options_availability: Option<Vec<OptionsAvailabilityAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionsAvailabilityAggregation {
    pub options: Option<Vec<OptionValueAggregation>>,
    pub available_quantity: Option<f64>,
    pub price_difference: Option<f64>,
    pub discount_percentage: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionValueAggregation {
    pub option: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetProductOptionsAvailabilityError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductOptionsAvailabilityError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductOptionsAvailabilityError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductOptionsAvailabilityError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
