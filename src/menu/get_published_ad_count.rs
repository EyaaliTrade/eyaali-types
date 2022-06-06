use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPublishedAdCountBody {
    pub company: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPublishedAdCountResult {
    pub count: i32,
}

#[derive(Debug, Display)]
pub enum GetPublishedAdCountError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetPublishedAdCountError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetPublishedAdCountError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetPublishedAdCountError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
