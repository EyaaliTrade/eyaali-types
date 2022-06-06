use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DuplicateSubCategoryBody {
    pub template_category: String,
    pub parent_category: String,
    pub template_id: String,
    pub menu: String,
    pub languages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DuplicateSubCategoryResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum DuplicateSubCategoryError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_template_category_not_found")]
    TemplateCategoryNotFound,
    Default(String),
}

impl ResponseError for DuplicateSubCategoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DuplicateSubCategoryError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DuplicateSubCategoryError::TemplateCategoryNotFound => {
                HttpResponse::NotFound().body("menu_template_category_not_found")
            }
            DuplicateSubCategoryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}