use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteProductAttributeBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteProductAttributeResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteProductAttributeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteProductAttributeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteProductAttributeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteProductAttributeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}