use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuLegalitySettingsBody {
    pub menu: String,
    pub language_code: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuLegalitySettingsResult {
    pub pages: Option<Vec<PageAggregation>>,
    pub consent_is_required: Option<bool>,
    pub approval_is_required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegalitySettingsAggregation {
    pub pages: Option<Vec<PageAggregation>>,
    pub consent_is_required: Option<bool>,
    pub approval_is_required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub title: Option<Vec<NameAggregation>>,
    pub content: Option<Vec<DescriptionAggregation>>,
    pub styles: Option<Vec<String>>,
    pub is_published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetMenuLegalitySettingsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_not_found")]
    MenuNotFound,
    Default(String),
}

impl ResponseError for GetMenuLegalitySettingsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuLegalitySettingsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuLegalitySettingsError::MenuNotFound => {
                HttpResponse::NotFound().body("menu_not_found")
            }
            GetMenuLegalitySettingsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}