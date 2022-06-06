use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetOptionValueNamesBody {
    pub product: Option<String>,
    pub option: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetOptionValueNamesResult {
    pub names: Option<Vec<NameAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetOptionValueNamesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetOptionValueNamesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetOptionValueNamesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetOptionValueNamesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
