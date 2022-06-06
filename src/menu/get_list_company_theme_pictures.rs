use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCompanyThemePicturesBody {
    pub company: Option<String>,
    pub kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCompanyThemePicturesResult {
    pub list: Vec<String>,
}

#[derive(Debug, Display)]
pub enum GetListCompanyThemePicturesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListCompanyThemePicturesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCompanyThemePicturesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListCompanyThemePicturesError::Default(error) => {
                HttpResponse::BadRequest().body(error)
            }
        }
    }
}
