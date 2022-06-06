use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateMenuCategoryBody {
    pub menu: String,
    pub identifier: Option<String>,
    pub has_multi_languages: Option<bool>,
    pub names: Option<Vec<CreateMenuCategoryNameBody>>,
    pub descriptions: Option<Vec<CreateMenuCategoryDescriptionBody>>,
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
pub struct CreateMenuCategoryNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuCategoryDescriptionBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCategoryCopiedPictureBody {
    pub id: Option<String>,
    pub quality: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuCategoryResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateMenuCategoryError {
    #[display(fmt = "required_main_category")]
    RequiredMainCategory,
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    #[display(fmt = "identifier_is_not_alphabectic")]
    IdentifierIsNotAplhabetic,
    Default(String),
}

impl ResponseError for CreateMenuCategoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateMenuCategoryError::RequiredMainCategory => {
                HttpResponse::Conflict().body("required_main_category")
            }
            CreateMenuCategoryError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            CreateMenuCategoryError::IdentifierIsNotAplhabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabectic")
            }
            CreateMenuCategoryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
