use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone,Validate)]
pub struct GetMenuStaticPageBody {
    pub id: Option<String>,
    pub identifier: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone,Validate)]
pub struct GetMenuStaticPageResult {
    pub static_page: Option<StaticPageAggregation>
}

#[derive(Serialize, Deserialize, Debug, Clone,Validate)]
pub struct StaticPageAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub title: Option<Vec<NameAggregation>>,
    pub content: Option<Vec<NameAggregation>>,
    pub styles: Option<Vec<String>>,
    pub is_published: Option<bool>,
    pub is_deleted: Option<bool>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetMenuStaticPageError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetMenuStaticPageError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuStaticPageError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuStaticPageError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
