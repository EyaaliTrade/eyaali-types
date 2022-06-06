use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct BindUserMenuBody {
    pub user: String,
    pub device: String,
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BindUserMenuResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum BindUserMenuError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for BindUserMenuError {
    fn error_response(&self) -> HttpResponse {
        match self {
            BindUserMenuError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            BindUserMenuError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
