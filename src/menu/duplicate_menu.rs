use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DuplicateMenuBody {
    pub template_id: String,
    pub company: String,
    pub identifier: String,
    pub names: Option<Vec<DuplicateMenuNameBody>>,
    pub currency: Option<String>,
    pub languages: Vec<DuplicateMenuLanguageBody>,
    pub characteristics: Option<Vec<MenuCharacteristicBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DuplicateMenuNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCharacteristicBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DuplicateMenuLanguageBody {
    pub language: Option<String>,
    pub is_main: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DuplicateMenuResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum DuplicateMenuError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    #[display(fmt = "menu_template_not_found")]
    MenuTemplateNotFound,
    Default(String),
}

impl ResponseError for DuplicateMenuError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DuplicateMenuError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DuplicateMenuError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            DuplicateMenuError::MenuTemplateNotFound => {
                HttpResponse::NotFound().body("menu_template_theme_not_found")
            }
            DuplicateMenuError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
