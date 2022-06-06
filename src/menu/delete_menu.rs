use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DeleteMenuBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteMenuResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteMenuError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteMenuError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteMenuError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteMenuError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
