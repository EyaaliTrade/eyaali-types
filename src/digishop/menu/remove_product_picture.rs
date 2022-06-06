use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RemoveProductPictureBody {
    pub product: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveProductPictureResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum RemoveProductPictureError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    #[display(fmt = "picture_not_found")]
    PictureNotFound,
    Default(String),
}

impl ResponseError for RemoveProductPictureError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RemoveProductPictureError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            RemoveProductPictureError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            RemoveProductPictureError::PictureNotFound => {
                HttpResponse::Conflict().body("picture_not_found")
            }
            RemoveProductPictureError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
