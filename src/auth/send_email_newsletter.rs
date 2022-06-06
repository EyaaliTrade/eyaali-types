use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SendEmailNewsletterBody {
    pub menu: String,
    pub subject: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendEmailNewsletterResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum SendEmailNewsletterError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for SendEmailNewsletterError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SendEmailNewsletterError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SendEmailNewsletterError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
