use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuCategoryDictBody {
    pub id: String,
    pub names: Option<Vec<UpdateMenuCategoryDictNameBody>>,
    pub descriptions: Option<Vec<UpdateMenuCategoryDictDescriptionBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuCategoryDictNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuCategoryDictDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuCategoryDictResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuCategoryDictError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_category_not_found")]
    MenuCategoryNotFound,
    Default(String),
}

impl ResponseError for UpdateMenuCategoryDictError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuCategoryDictError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuCategoryDictError::MenuCategoryNotFound => {
                HttpResponse::NotFound().body("menu_category_not_found")
            }
            UpdateMenuCategoryDictError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}