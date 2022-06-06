use crate::menu::get_list_menu_categories::{
    MenuCategoryDescriptionAggregation, MenuCategoryNameAggregation,
};
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessMenuSubCategoryDetailsBody {
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessMenuSubCategoryDetailsResult {
    pub menu_sub_category: Option<AccessMenuSubCategoryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessMenuSubCategoryAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<MenuCategoryNameAggregation>>,
    pub descriptions: Option<Vec<MenuCategoryDescriptionAggregation>>,
    pub picture: Option<MenuCategoryPictureAggregation>,
    pub copied_pictures: Option<Vec<MenuCategoryPictureAggregation>>,
    pub picture_is_visible: Option<bool>,
    pub display: Option<String>,
    pub order: Option<i32>,
    pub is_published: Option<bool>,
    pub products: Option<Vec<CategoryProductAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCategoryPictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductAggregation {
    pub id: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetAccessMenuSubCategoryError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetAccessMenuSubCategoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAccessMenuSubCategoryError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetAccessMenuSubCategoryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
