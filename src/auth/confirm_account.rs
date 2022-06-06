use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfirmAccountBody {
    pub verification_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfirmAccountResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum ConfirmAccountError {
    #[display(fmt = "code_not_found")]
    CodeNotFound,
    Default(String),
}

impl ResponseError for ConfirmAccountError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ConfirmAccountError::CodeNotFound => HttpResponse::NotFound().json("code_not_found"),
            ConfirmAccountError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
