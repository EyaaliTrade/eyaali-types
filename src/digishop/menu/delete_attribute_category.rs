use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteAttributeCategoryBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteAttributeCategoryResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteAttributeCategoryError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteAttributeCategoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteAttributeCategoryError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteAttributeCategoryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}