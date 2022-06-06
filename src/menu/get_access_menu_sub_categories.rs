use crate::menu::get_access_menu_sub_category::AccessMenuSubCategoryAggregation;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessMenuSubCategoriesBody {
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessMenuSubCategoriesResult {
    list: Option<Vec<AccessMenuSubCategoryAggregation>>,
}

#[derive(Debug, Display)]
pub enum GetAccessMenuSubCategoriesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetAccessMenuSubCategoriesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAccessMenuSubCategoriesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetAccessMenuSubCategoriesError::Default(error) => {
                HttpResponse::BadRequest().body(error)
            }
        }
    }
}
