use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAuthorizedAdCountBody {
    pub company: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAuthorizedAdCountResult {
    pub count: i32,
}

#[derive(Debug, Display)]
pub enum GetAuthorizedAdCountError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetAuthorizedAdCountError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAuthorizedAdCountError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetAuthorizedAdCountError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
