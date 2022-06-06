use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SetMenuCategoriesOrderBody {
    pub categories: Option<Vec<MenuCategoryOrderBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCategoryOrderBody {
    pub id: String,
    pub order: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetMenuCategoriesOrderResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum SetMenuCategoriesOrderError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for SetMenuCategoriesOrderError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SetMenuCategoriesOrderError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SetMenuCategoriesOrderError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
