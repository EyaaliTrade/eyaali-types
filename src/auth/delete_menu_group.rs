use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DeleteMenuGroupBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteMenuGroupResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteMenuGroupError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteMenuGroupError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteMenuGroupError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteMenuGroupError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
