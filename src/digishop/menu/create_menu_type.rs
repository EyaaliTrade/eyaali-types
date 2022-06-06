use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateMenuTypeBody {
    pub menu: String,
    pub identifier: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuTypeResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateMenuTypeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for CreateMenuTypeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateMenuTypeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateMenuTypeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
