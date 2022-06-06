use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone,Validate)]
pub struct UpdateStaticPageBody {
    pub id: String,
    pub menu: String,
    pub identifier: Option<String>,
    pub title: Option<Vec<UpdateNameBody>>,
    pub content: Option<Vec<UpdateNameBody>>,
    pub styles: Option<Vec<String>>,
    pub is_published: Option<bool>,
    pub is_deleted: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone,Validate)]
pub struct UpdateStaticPageResult {
    pub success: bool
}

#[derive(Debug, Display)]
pub enum UpdateStaticPageError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "static_page_not_found")]
    StaticPageNotFound,
    Default(String),
}

impl ResponseError for UpdateStaticPageError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateStaticPageError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateStaticPageError::StaticPageNotFound => {
                HttpResponse::NotFound().body("static_page_exists")
            }
            UpdateStaticPageError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
