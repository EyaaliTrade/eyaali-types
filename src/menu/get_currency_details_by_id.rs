use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCurrencyDetailsByIdBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCurrencyDetailsByIdResult {
    pub currency: Option<CurrencyAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetCurrencyDetailsByIdError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetCurrencyDetailsByIdError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCurrencyDetailsByIdError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCurrencyDetailsByIdError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
