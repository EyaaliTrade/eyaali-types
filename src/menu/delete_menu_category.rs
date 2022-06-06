use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DeleteMenuCategoryBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteMenuCategoryResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteMenuCategoryError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteMenuCategoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteMenuCategoryError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteMenuCategoryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
