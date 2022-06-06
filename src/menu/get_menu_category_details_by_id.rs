use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuCategoryDetailsByIdBody {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub menu: Option<String>,
    pub company: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuCategoryDetailsByIdResult {
    pub menu_category: Option<MenuCategoryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCategoryAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<MenuCategoryNameAggregation>>,
    pub descriptions: Option<Vec<MenuCategoryDescriptionAggregation>>,
    pub picture: Option<MenuCategoryPictureAggregation>,
    pub copied_pictures: Option<Vec<MenuCategoryPictureAggregation>>,
    pub level: Option<i32>,
    pub parent: Option<String>,
    pub order: Option<i32>,
    pub picture_is_visible: Option<bool>,
    pub display: Option<String>,
    pub products: Option<Vec<MenuCategoryProductAggregation>>,
    pub sub_categories: Option<Vec<MenuSubCategoryAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCategoryProductAggregation {
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuSubCategoryAggregation {
    pub id: Option<String>,
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
    pub file_url: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetMenuCategoryDetailsByIdError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetMenuCategoryDetailsByIdError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuCategoryDetailsByIdError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuCategoryDetailsByIdError::Default(error) => {
                HttpResponse::BadRequest().body(error)
            }
        }
    }
}
