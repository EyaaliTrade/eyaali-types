use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductAncestorsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryAggregation {
    pub category_id: Option<String>,
    pub name: Option<String>,
    pub identifier: Option<String>,
    pub level: Option<i32>,
    pub parent: Option<String>,
    pub main: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductAncestorsResult {
    pub ancestors: Vec<ProductCategoryAggregation>
}

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd,Clone,)]
pub struct ProductCategoryAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
    pub identifier: Option<String>,
    pub level: Option<i32>,
}

#[derive(Debug, Display)]
pub enum GetProductAncestorsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for GetProductAncestorsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductAncestorsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductAncestorsError::ProductNotFound => {
                HttpResponse::NotFound().body("product_not_found")
            }
            GetProductAncestorsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}