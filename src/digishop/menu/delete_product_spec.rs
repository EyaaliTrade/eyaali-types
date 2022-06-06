use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteProductSpecBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteProductSpecResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteProductSpecError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteProductSpecError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteProductSpecError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteProductSpecError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
