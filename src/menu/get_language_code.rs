use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLanguageCodeBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLanguageCodeResult {
    pub code: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetLanguageCodeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetLanguageCodeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetLanguageCodeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetLanguageCodeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}