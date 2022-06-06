use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListLanguagesResult {
    pub list: Vec<LanguageAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguageAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListLanguagesError {
    Default(String),
}

impl ResponseError for GetListLanguagesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListLanguagesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
