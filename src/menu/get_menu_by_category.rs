use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuByCategoryBody {
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuByCategoryResult {
    pub menu: String,
}

#[derive(Debug, Display)]
pub enum GetMenuByCategoryError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetMenuByCategoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuByCategoryError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuByCategoryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
