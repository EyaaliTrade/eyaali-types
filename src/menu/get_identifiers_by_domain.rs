use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIdentifiersByDomainBody {
    pub company: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIdentifiersByDomainResult {
    pub list: Vec<MenuIdentifierByDomainAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuIdentifierByDomainAggregation {
    pub menu: Option<String>,
    pub languages: Option<Vec<MenuLanguageAggregation>>,
    pub categories: Option<Vec<CategoryIdentifierByDomainAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuLanguageAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub is_main: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryIdentifierByDomainAggregation {
    pub identifier: Option<String>,
    pub products: Option<Vec<ProductIdentifierByDomainAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductIdentifierByDomainAggregation {
    pub identifier: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetIdentifiersByDomainError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetIdentifiersByDomainError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetIdentifiersByDomainError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetIdentifiersByDomainError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
