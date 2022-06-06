use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateFileKindBody {
    pub id: String,
    pub new_kind: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateFileKindResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateFileKindError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "file_not_found")]
    FileNotFound,
    Default(String),
}

impl ResponseError for UpdateFileKindError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateFileKindError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateFileKindError::FileNotFound => {
                HttpResponse::NotFound().body("file_not_found")
            }
            UpdateFileKindError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
