use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateFileDetailsBody {
    pub id: String,
    pub details: Option<Vec<FileDetailsBody>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileDetailsBody {
    pub kind: Option<String>,
    pub value: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateFileDetailsResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateFileDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "file_not_found")]
    FileNotFound,
    Default(String),
}

impl ResponseError for UpdateFileDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateFileDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateFileDetailsError::FileNotFound => {
                HttpResponse::NotFound().body("file_not_found")
            }
            UpdateFileDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
