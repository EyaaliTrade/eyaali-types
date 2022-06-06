use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryHierarchyBody {
    pub id: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryHierarchyResult {
    pub categories: Vec<CategoryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub level: Option<i32>,
    pub children: Option<Vec<CategoryAggregation>>,
    pub has_children: Option<i32>
}

#[derive(Debug, Display)]
pub enum GetCategoryHierarchyError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "category_not_found")]
    CategoryNotFound,
    Default(String),
}

impl ResponseError for GetCategoryHierarchyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCategoryHierarchyError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCategoryHierarchyError::CategoryNotFound => {
                HttpResponse::NotFound().body("category_not_found")
            }
            GetCategoryHierarchyError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}