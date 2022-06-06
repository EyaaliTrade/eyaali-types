use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuCategoryBody {
    pub id: String,
    pub menu: Option<String>,
    pub identifier: Option<String>,
    pub has_multi_languages: Option<bool>,
    pub names: Option<Vec<UpdateMenuCategoryNameBody>>,
    pub descriptions: Option<Vec<UpdateMenuCategoryDescriptionBody>>,
    pub level: Option<i32>,
    pub main: Option<String>,
    pub parent: Option<String>,
    pub path: Option<String>,
    pub picture: Option<String>,
    pub copied_pictures: Option<Vec<MenuCategoryCopiedPictureBody>>,
    pub display: Option<String>,
    pub is_published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuCategoryNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuCategoryDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCategoryCopiedPictureBody {
    pub id: Option<String>,
    pub quality: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuCategoryResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuCategoryError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_category_not_found")]
    MenuCategoryNotFound,
    #[display(fmt = "required_main_category")]
    RequiredMainCategory,
    #[display(fmt = "identifier_is_not_alphabectic")]
    IdentifierIsNotAplhabetic,
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    Default(String),
}

impl ResponseError for UpdateMenuCategoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuCategoryError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuCategoryError::MenuCategoryNotFound => {
                HttpResponse::Conflict().body("menu_category_not_found")
            }
            UpdateMenuCategoryError::RequiredMainCategory => {
                HttpResponse::Conflict().body("required_main_category")
            }
            UpdateMenuCategoryError::IdentifierIsNotAplhabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabectic")
            }
            UpdateMenuCategoryError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            UpdateMenuCategoryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
