use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCategoriesBody {
    pub menu: String,
    pub level: Option<i32>,
    pub parent: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCategoriesResult {
    pub list: Vec<CategoryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<CategoryNameAggregation>>,
    pub level: Option<i32>,
    pub parent: Option<String>,
    pub display: Option<String>,
    pub picture: Option<CategoryPictureUrlAggregation>,
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
pub enum GetListCategoriesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListCategoriesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCategoriesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListCategoriesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}