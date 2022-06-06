use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePermissionBody {
    pub kind: Option<String>,
    pub names: Option<Vec<PermissionNameBody>>,
    pub descriptions: Option<Vec<PermissionDesccriptionBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionNameBody {
    pub language: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionDesccriptionBody {
    pub language: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePermissionResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreatePermissionError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for CreatePermissionError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreatePermissionError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreatePermissionError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
