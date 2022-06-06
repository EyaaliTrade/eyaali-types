use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetAccessPointSettingsBody {
    pub access_point: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessPointSettingsResult {
    pub settings: Option<AccessPointSettingsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessPointSettingsAggregation {
    pub id: Option<String>,
    pub access_point: Option<String>,
    pub registration_type: Option<Vec<String>>,
    pub authentication_type: Option<String>,
    pub session_duration: Option<i32>,
    pub debit_limit: Option<i32>,
    pub settings_characteristics: Option<Vec<SettingsCharacteristic>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsCharacteristic {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetAccessPointSettingsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetAccessPointSettingsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAccessPointSettingsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetAccessPointSettingsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
