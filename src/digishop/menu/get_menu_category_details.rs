use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuCategoryDetailsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuCategoryDetailsResult {
    pub category: MenuCategoryDetailsAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCategoryDetailsAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub has_multi_languages: Option<bool>,
    pub names: Option<Vec<MenuCategoryNameAggregation>>,
    pub descriptions: Option<Vec<MenuCategoryDescriptionAggregation>>,
    pub level: Option<i32>,
    pub main: Option<String>,
    pub parent: Option<String>,
    pub picture: Option<MenuCategoryPictureUrlAggregation>,
    pub display: Option<String>,
    pub is_published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCategoryNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCategoryDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCategoryPictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetMenuCategoryDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "category_not_found")]
    CategoryNotFound,
    Default(String),
}

impl ResponseError for GetMenuCategoryDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuCategoryDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuCategoryDetailsError::CategoryNotFound => {
                HttpResponse::Conflict().body("category_not_found")
            }
            GetMenuCategoryDetailsError::Default(error) => {
                HttpResponse::BadRequest().body(error)
            }
        }
    }
}
