use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DuplicateMenuCategoryBody {
    pub template_category: String,
    pub template_id: String,
    pub menu: String,
    pub languages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DuplicateMenuCategoryResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum DuplicateMenuCategoryError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_template_category_not_found")]
    TemplateCategoryNotFound,
    Default(String),
}

impl ResponseError for DuplicateMenuCategoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DuplicateMenuCategoryError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DuplicateMenuCategoryError::TemplateCategoryNotFound => {
                HttpResponse::NotFound().body("menu_template_category_not_found")
            }
            DuplicateMenuCategoryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
