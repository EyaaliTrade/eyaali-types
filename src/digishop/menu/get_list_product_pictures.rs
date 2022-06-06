use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductPicturesBody {
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductPicturesResult {
    pub list: Vec<ProductPictureAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPictureAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPicturesAggregation {
    pub pictures: Vec<ProductPictureAggregation>,
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
