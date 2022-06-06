use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCompanyByMenuBody {
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCompanyByMenuResult {
    pub company: String,
}

#[derive(Debug, Display)]
pub enum GetCompanyByMenuError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetCompanyByMenuError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCompanyByMenuError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCompanyByMenuError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
