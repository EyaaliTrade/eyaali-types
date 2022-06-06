use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductOptionNamesBody {
    pub product: Option<String>,
    pub option: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductOptionNamesResult {
    pub names: Option<Vec<NameAggregation>>,
    pub input_kind: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetProductOptionNamesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductOptionNamesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductOptionNamesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductOptionNamesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
