use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DuplicateMenuAdsBody {
    pub template_id: String,
    pub company: String,
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DuplicateMenuAdsResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DuplicateMenuAdsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_template_not_found")]
    MenuTemplateNotFound,
    Default(String),
}

impl ResponseError for DuplicateMenuAdsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DuplicateMenuAdsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DuplicateMenuAdsError::MenuTemplateNotFound => {
                HttpResponse::NotFound().body("menu_template_theme_not_found")
            }
            DuplicateMenuAdsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
