use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuPicturesBody {
    pub menu_access_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuPicturesResult {
    pub list: Vec<String>,
}

#[derive(Debug, Display)]
pub enum GetListMenuPicturesError {
    #[display(fmt = "menu_not_found")]
    MenuNotFound,
    Default(String),
}

impl ResponseError for GetListMenuPicturesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMenuPicturesError::MenuNotFound => {
                HttpResponse::Conflict().body("menu_not_found")
            }
            GetListMenuPicturesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
