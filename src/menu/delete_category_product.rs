use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DeleteCategoryProductBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteCategoryProductResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteCategoryProductError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteCategoryProductError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteCategoryProductError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteCategoryProductError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
