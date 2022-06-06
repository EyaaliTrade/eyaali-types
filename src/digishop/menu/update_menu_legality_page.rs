//LIB
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuLegalityPageBody {
    pub id: String,
    pub menu: String,
    pub identifier: Option<String>,
    pub title: Option<Vec<UpdateTitleBody>>,
    pub content: Option<Vec<UpdateContentBody>>,
    pub styles: Option<Vec<String>>,
    pub is_published: Option<bool>,
    pub is_deleted: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTitleBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateContentBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuLegalityPageResult {
    pub success: bool
}

#[derive(Debug, Display)]
pub enum UpdateMenuLegalityPageError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "static_page_not_found")]
    LegalityPageNotFound,
    Default(String),
}

impl ResponseError for UpdateMenuLegalityPageError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuLegalityPageError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuLegalityPageError::LegalityPageNotFound => {
                HttpResponse::NotFound().body("static_page_exists")
            }
            UpdateMenuLegalityPageError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
