use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuLegalitySettingsBody {
    pub menu: String,
    pub pages: Option<Vec<UpdateMenuLegalityPageBody>>,
    pub consent_is_required: Option<bool>,
    pub approval_is_required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuLegalityPageBody {
    pub identifier: String,
    pub is_published: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuLegalitySettingsResult {
    pub success: bool
}

#[derive(Debug, Display)]
pub enum UpdateMenuLegalitySettingsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_not_found")]
    MenuNotFound,
    Default(String),
}

impl ResponseError for UpdateMenuLegalitySettingsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuLegalitySettingsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuLegalitySettingsError::MenuNotFound => {
                HttpResponse::NotFound().body("menu_exists")
            }
            UpdateMenuLegalitySettingsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
