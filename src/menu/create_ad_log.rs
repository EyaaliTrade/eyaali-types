use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateAdLogBody {
    pub ad: Option<String>,
    pub kind: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAdLogResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateAdLogError {
    Default(String),
}

impl ResponseError for CreateAdLogError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateAdLogError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
