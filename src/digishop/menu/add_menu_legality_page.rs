//LIB
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddMenuLegalityPageBody {
    pub menu: String,
    pub identifier: Option<String>,
    pub title: Option<Vec<AddTitleBody>>,
    pub content: Option<Vec<AddContentBody>>,
    pub styles: Option<Vec<String>>,
    pub is_published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddTitleBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddContentBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddMenuLegalityPageResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum AddMenuLegalityPageError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    #[display(fmt = "identifier_is_not_alphabetic")]
    IdentifierIsNotAlphabetic,
    Default(String),
}

impl ResponseError for AddMenuLegalityPageError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AddMenuLegalityPageError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            AddMenuLegalityPageError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            AddMenuLegalityPageError::IdentifierIsNotAlphabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabetic")
            }
            AddMenuLegalityPageError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}