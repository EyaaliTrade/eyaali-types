use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteFileBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteFileResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteFileError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "file_not_found")]
    FileNotFound,
    Default(String),
}

impl ResponseError for DeleteFileError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteFileError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteFileError::FileNotFound => {
                HttpResponse::NotFound().body("file_not_found")
            }
            DeleteFileError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
