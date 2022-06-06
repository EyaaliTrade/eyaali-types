use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetOptionsAvailableQuantityBody {
    pub product: String,
    pub options: Vec<OptionValueBody>,
    pub site: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionValueBody {
    pub option: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetOptionsAvailableQuantityResult {
    pub available_quantity: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionsAvailableQuantityAggregation {
    pub available_quantity: Option<f64>,
}

#[derive(Debug, Display)]
pub enum GetOptionsAvailableQuantityError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetOptionsAvailableQuantityError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetOptionsAvailableQuantityError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetOptionsAvailableQuantityError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
