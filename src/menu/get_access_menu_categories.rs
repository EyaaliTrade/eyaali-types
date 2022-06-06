use crate::menu::get_list_menu_categories::MenuCategoryAggregation;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessMenuCategoriesBody {
    pub menu: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessMenuCategoriesResult {
    pub list: Option<Vec<MenuCategoryAggregation>>,
}

#[derive(Debug, Display)]
pub enum GetAccessMenuCategoriesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetAccessMenuCategoriesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAccessMenuCategoriesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetAccessMenuCategoriesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
