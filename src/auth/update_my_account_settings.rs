use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMyAccountSettingsBody {
    pub id: String,
    pub language: Option<String>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMyAccountSettingsResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMyAccountSettingsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    Default(String),
}

impl ResponseError for UpdateMyAccountSettingsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMyAccountSettingsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMyAccountSettingsError::UserNotFound => {
                HttpResponse::Conflict().body("user_not_found")
            }
            UpdateMyAccountSettingsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
