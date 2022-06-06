use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetEasyPayListCategoriesResult {
    pub list: Vec<EasyPayCategoryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EasyPayCategoryAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetEasyPayListCategoriesError {
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetEasyPayListCategoriesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetEasyPayListCategoriesError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetEasyPayListCategoriesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
