
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryIdByIdentifierBody {
    pub identifier: String,
    pub menu: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryIdAggregation {
    pub id: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryIdByIdentifierResult {
    pub id: String,
}


#[derive(Debug, Display)]
pub enum GetCategoryIdByIdentifierError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_category_not_found")]
    CategoryNotFound,
    Default(String),
}

impl ResponseError for GetCategoryIdByIdentifierError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCategoryIdByIdentifierError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCategoryIdByIdentifierError::CategoryNotFound => {
                HttpResponse::NotFound().body("menu_category_not_found")
            }
            GetCategoryIdByIdentifierError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}