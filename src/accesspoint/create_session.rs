use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateSessionBody {
    pub user_id: String,
    pub access_point: String,
    pub characteristics: Option<Vec<SessionCharacteristicBody>>,
    pub session_type: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSessionTitleBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionCharacteristicBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSessionResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateSessionError {
    Default(String),
}

impl ResponseError for CreateSessionError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateSessionError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
