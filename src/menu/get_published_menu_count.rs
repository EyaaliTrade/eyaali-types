use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPublishedMenuCountBody {
    pub company: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPublishedMenuCountResult {
    pub count: i32,
}

#[derive(Debug, Display)]
pub enum GetPublishedMenuCountError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetPublishedMenuCountError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetPublishedMenuCountError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetPublishedMenuCountError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
