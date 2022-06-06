use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuTypeBody {
    pub id: String,
    pub description: Option<UpdateMenuTypeDescriptionBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuTypeDescriptionBody {
    pub id: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuTypeResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuTypeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_type_not_found")]
    MenuTypeNotFound,
    Default(String),
}

impl ResponseError for UpdateMenuTypeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuTypeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuTypeError::MenuTypeNotFound => {
                HttpResponse::Conflict().body("menu_type_not_found")
            }
            UpdateMenuTypeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
