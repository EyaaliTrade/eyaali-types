use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMyAccountSettingsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMyAccountSettingsResult {
    pub account_settings: AccountSettingsAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountSettingsAggregation {
    pub language: Option<String>,
    pub currency: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetMyAccountSettingsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    Default(String),
}

impl ResponseError for GetMyAccountSettingsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMyAccountSettingsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMyAccountSettingsError::UserNotFound => {
                HttpResponse::Conflict().body("user_not_found")
            }
            GetMyAccountSettingsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
