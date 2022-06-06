use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DeleteMenuThemeBody {
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteMenuThemeResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteMenuThemeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteMenuThemeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteMenuThemeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteMenuThemeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
