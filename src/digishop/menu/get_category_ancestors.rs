use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryAncestorsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentCategoryAggregation {
    pub category_id: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
    pub identifier: Option<String>,
    pub level: Option<i32>,
    pub parent: Option<String>,
    pub main: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryAncestorsResult {
    pub ancestors: Vec<CategoryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryAggregation {
    pub id: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
    pub identifier: Option<String>,
    pub level: Option<i32>,
}

#[derive(Debug, Display)]
pub enum GetCategoryAncestorsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "category_not_found")]
    CategoryNotFound,
    Default(String),
}

impl ResponseError for GetCategoryAncestorsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCategoryAncestorsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCategoryAncestorsError::CategoryNotFound => {
                HttpResponse::NotFound().body("category_not_found")
            }
            GetCategoryAncestorsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
