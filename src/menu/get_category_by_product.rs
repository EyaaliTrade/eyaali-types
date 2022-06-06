use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryByProductBody {
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryByProductResult {
    pub category: String,
}

#[derive(Debug, Display)]
pub enum GetCategoryByProductError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetCategoryByProductError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCategoryByProductError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCategoryByProductError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
