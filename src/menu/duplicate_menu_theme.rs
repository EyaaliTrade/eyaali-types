use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DuplicateMenuThemeBody {
    pub menu_template: String,
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DuplicateMenuThemeResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum DuplicateMenuThemeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_template_theme_not_found")]
    TemplateThemeNotFound,
    Default(String),
}

impl ResponseError for DuplicateMenuThemeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DuplicateMenuThemeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DuplicateMenuThemeError::TemplateThemeNotFound => {
                HttpResponse::NotFound().body("menu_template_theme_not_found")
            }
            DuplicateMenuThemeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
