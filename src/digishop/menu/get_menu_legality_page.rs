use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuLegalityPageBody {
    pub identifier: String,
    pub menu: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuLegalityPageResult {
    pub page: Option<LegalityPageAggregation>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegalityPageAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub title: Option<Vec<NameAggregation>>,
    pub content: Option<Vec<DescriptionAggregation>>,
    pub styles: Option<Vec<String>>,
    pub is_published: Option<bool>,
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

#[derive(Debug, Display)]
pub enum GetMenuLegalityPageError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetMenuLegalityPageError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuLegalityPageError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuLegalityPageError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
