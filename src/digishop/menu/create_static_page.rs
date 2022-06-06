use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone,Validate)]
pub struct CreateStaticPageBody {
    pub menu: String,
    pub identifier: Option<String>,
    pub title: Option<Vec<CreateNameBody>>,
    pub content: Option<Vec<CreateNameBody>>,
    pub styles: Option<Vec<String>>,
    pub is_published: Option<bool>,
    pub is_deleted: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateStaticPageResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateStaticPageError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "static_page_exists")]
    StaticPageExists,
    Default(String),
}

impl ResponseError for CreateStaticPageError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateStaticPageError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateStaticPageError::StaticPageExists => {
                HttpResponse::Conflict().body("static_page_exists")
            }
            CreateStaticPageError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
