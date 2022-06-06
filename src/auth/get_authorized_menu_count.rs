use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAuthorizedMenuCountBody {
    pub company: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAuthorizedMenuCountResult {
    pub count: i32,
}

#[derive(Debug, Display)]
pub enum GetAuthorizedMenuCountError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetAuthorizedMenuCountError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAuthorizedMenuCountError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetAuthorizedMenuCountError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
