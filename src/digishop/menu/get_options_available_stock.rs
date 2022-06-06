use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetOptionsAvailableStockBody {
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
pub struct GetOptionsAvailableStockResult {
    pub list: Vec<OptionsAvailableStockAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionsAvailableStockAggregation {
    pub id: Option<String>,
    pub available_quantity: Option<f64>,
}

#[derive(Debug, Display)]
pub enum GetOptionsAvailableStockError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetOptionsAvailableStockError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetOptionsAvailableStockError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetOptionsAvailableStockError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
