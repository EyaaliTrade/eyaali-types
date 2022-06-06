use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateAccessPointSettingsBody {
    pub access_point: String,
    pub authentication_type: Option<String>,
    pub registration_type: Option<Vec<String>>,
    pub session_duration: Option<i32>,
    pub debit_limit: Option<i32>,
    pub settings_characteristics: Option<Vec<CreateSettingsCharacteristicBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSettingsCharacteristicBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAccessPointSettingsResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateAccessPointSettingsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "access_point_not_found")]
    AccessPointNotFound,
    Default(String),
}

impl ResponseError for CreateAccessPointSettingsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateAccessPointSettingsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateAccessPointSettingsError::AccessPointNotFound => {
                HttpResponse::NotFound().body("access_point_not_found")
            }
            CreateAccessPointSettingsError::Default(error) => {
                HttpResponse::BadRequest().body(error)
            }
        }
    }
}
