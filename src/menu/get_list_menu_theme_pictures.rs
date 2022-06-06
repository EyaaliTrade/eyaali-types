use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuThemePicturesBody {
    pub kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuThemePicturesResult {
    pub list: Vec<String>,
}

#[derive(Debug, Display)]
pub enum GetListMenuThemePicturesError {
    Default(String),
}

impl ResponseError for GetListMenuThemePicturesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMenuThemePicturesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
