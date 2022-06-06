use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteMenuTypeBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteMenuTypeResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteMenuTypeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteMenuTypeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteMenuTypeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteMenuTypeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}