use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductPicturesBody {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub company: Option<String>,
    pub menu: Option<String>,
    pub category: Option<String>,
    pub quality: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductPicturesResult {
    pub list: Vec<ProductKindPictureAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductKindPictureAggregation {
    pub kind: Option<String>,
    pub picture: Option<ProductPictureAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListProductPicturesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListProductPicturesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListProductPicturesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListProductPicturesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
