use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuGalleryBody {
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuGalleryResult {
    pub gallery: Option<Vec<FileAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileAggregation {
    pub id: Option<String>,
    pub file_name: Option<String>,
    pub kind: Option<String>,
    pub extension: Option<String>,
    pub details: Option<Vec<FileDetailsAggregation>>,
    pub file_url: Option<String>,
    pub uploaded_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileDetailsAggregation {
    pub kind: Option<String>,
    pub value: Option<String>
}


#[derive(Debug, Display)]
pub enum GetMenuGalleryError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetMenuGalleryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuGalleryError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuGalleryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
