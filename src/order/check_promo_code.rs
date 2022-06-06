use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CheckPromoCodeBody {
    pub user: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckPromoCodeResult {
    pub cost: f64,
}

#[derive(Debug, Display)]
pub enum CheckPromoCodeError {
    #[display(fmt = "code_not_available")]
    CodeNotAvailable,
    Default(String),
}

impl ResponseError for CheckPromoCodeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CheckPromoCodeError::CodeNotAvailable => HttpResponse::Conflict().body("code_not_available"),
            CheckPromoCodeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
