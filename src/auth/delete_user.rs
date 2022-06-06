use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DeleteUserBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteUserResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteUserError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteUserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteUserError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteUserError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
