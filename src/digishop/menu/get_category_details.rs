//LIB
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryDetailsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryDetailsResult {
    pub category: CategoryDetailsAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryDetailsAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
    pub descriptions: Option<Vec<DescriptionAggregation>>,
    pub picture: Option<PictureAggregation>,
    pub copied_pictures: Option<Vec<PictureAggregation>>,
    pub display: Option<String>,
    pub sub_categories: Option<Vec<CategoryDetailsAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PictureAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetCategoryDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "category_not_found")]
    CategoryNotFound,
    Default(String),
}

impl ResponseError for GetCategoryDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCategoryDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCategoryDetailsError::CategoryNotFound => {
                HttpResponse::NotFound().body("category_not_found")
            }
            GetCategoryDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
