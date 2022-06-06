use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCurrenciesResult {
    pub list: Vec<CurrencyAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListCurrenciesError {
    Default(String),
}

impl ResponseError for GetListCurrenciesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCurrenciesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
