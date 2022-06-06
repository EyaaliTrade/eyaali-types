use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuCategoriesBody {
    pub menu: String,
    pub level: Option<i32>,
    pub parent: Option<String>,
    pub name: Option<String>,
    pub language_code: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuCategoriesResult {
    pub list: Vec<MenuCategoryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCategoryAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<CategoryNameAggregation>>,
    pub descriptions: Option<Vec<CategoryNameAggregation>>,
    pub level: Option<i32>,
    pub main: Option<String>,
    pub parent: Option<String>,
    pub picture: Option<CategoryPictureUrlAggregation>,
    pub order: Option<i32>,
    pub is_published: Option<bool>,
    pub sub_categories_count: Option<i32>,
    pub products_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryPictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
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
