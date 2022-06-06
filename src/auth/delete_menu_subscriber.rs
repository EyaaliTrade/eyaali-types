use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DeleteMenuSubscriberBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteMenuSubscriberResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteMenuSubscriberError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteMenuSubscriberError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteMenuSubscriberError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteMenuSubscriberError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
