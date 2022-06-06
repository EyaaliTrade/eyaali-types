use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DeleteCompanySiteBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteCompanySiteResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteCompanySiteError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteCompanySiteError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteCompanySiteError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteCompanySiteError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
