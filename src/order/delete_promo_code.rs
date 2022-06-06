use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DeletePromoCodeBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletePromoCodeResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeletePromoCodeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeletePromoCodeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeletePromoCodeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeletePromoCodeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
