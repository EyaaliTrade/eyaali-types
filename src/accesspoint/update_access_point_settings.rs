use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateAccessPointSettingsBody {
    pub access_point: String,
    pub authentication_type: Option<String>,
    pub registration_type: Option<Vec<String>>,
    pub session_duration: Option<i32>,
    pub debit_limit: Option<i32>,
    pub settings_characteristics: Option<Vec<UpdateSettingsCharacteristicBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSettingsCharacteristicBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAccessPointSettingsResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateAccessPointSettingsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "access_point_not_found")]
    AccessPointNotFound,
    Default(String),
}

impl ResponseError for UpdateAccessPointSettingsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateAccessPointSettingsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateAccessPointSettingsError::AccessPointNotFound => {
                HttpResponse::NotFound().body("access_point_not_found")
            }
            UpdateAccessPointSettingsError::Default(error) => {
                HttpResponse::BadRequest().body(error)
            }
        }
    }
}
