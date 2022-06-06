use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuCategoriesBody {
    pub menu: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuCategoriesResult {
    pub list: Vec<MenuCategoryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCategoryAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<MenuCategoryNameAggregation>>,
    pub descriptions: Option<Vec<MenuCategoryDescriptionAggregation>>,
    pub level: Option<i32>,
    pub parent: Option<String>,
    pub picture: Option<MenuCategoryPictureAggregation>,
    pub copied_pictures: Option<Vec<MenuCategoryPictureAggregation>>,
    pub picture_is_visible: Option<bool>,
    pub display: Option<String>,
    pub order: Option<i32>,
    pub is_published: Option<bool>,
    pub products_count: Option<i32>,
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
pub struct MenuCategoryPictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListMenuCategoriesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListMenuCategoriesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMenuCategoriesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListMenuCategoriesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
