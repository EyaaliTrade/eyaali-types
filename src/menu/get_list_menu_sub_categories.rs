use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuSubCategoriesBody {
    pub category: Option<String>,
    pub categories: Option<Vec<String>>,
    pub identifier: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuSubCategoriesResult {
    pub list: Vec<MenuSubCategoryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuSubCategoryAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<MenuSubCategoryNameAggregation>>,
    pub descriptions: Option<Vec<MenuSubCategoryDescriptionAggregation>>,
    pub level: Option<i32>,
    pub parent: Option<String>,
    pub picture: Option<MenuSubCategoryPictureAggregation>,
    pub copied_pictures: Option<Vec<MenuSubCategoryPictureAggregation>>,
    pub picture_is_visible: Option<bool>,
    pub display: Option<String>,
    pub order: Option<i32>,
    pub is_published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuSubCategoryNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuSubCategoryDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuSubCategoryPictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListMenuSubCategoriesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListMenuSubCategoriesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMenuSubCategoriesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListMenuSubCategoriesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
