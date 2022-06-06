use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AddProductPictureBody {
    pub product: String,
    pub picture: Option<String>,
    pub copied_pictures: Option<Vec<CategoryProductCopiedPictureBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductCopiedPictureBody {
    pub id: Option<String>,
    pub quality: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddProductPictureResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum AddProductPictureError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for AddProductPictureError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AddProductPictureError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            AddProductPictureError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            AddProductPictureError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
