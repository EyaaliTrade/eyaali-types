use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListAttributeCategoriesBody {
    pub menu_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListAttributeCategoriesResult {
    pub list: Vec<AttributeCategoryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeCategoryAggregation {
    pub id: Option<String>,
    pub name: Option<AttributeCategoryNameAggregation>,
    pub picture: Option<PictureUrlAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeCategoryNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListAttributeCategoriesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListAttributeCategoriesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListAttributeCategoriesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListAttributeCategoriesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
