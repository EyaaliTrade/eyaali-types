use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListFileKindsBody {
    pub language_code: String,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListFileKindsResult {
    pub list: Vec<FileKindAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileKindAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListFileKindsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListFileKindsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListFileKindsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListFileKindsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
