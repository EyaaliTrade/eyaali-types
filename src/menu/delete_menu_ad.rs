use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DeleteMenuAdBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteMenuAdResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteMenuAdError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteMenuAdError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteMenuAdError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteMenuAdError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
