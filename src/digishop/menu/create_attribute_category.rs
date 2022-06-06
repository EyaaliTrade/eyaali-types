use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateAttributeCategoryBody {
    pub menu_type: String,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub copied_pictures: Option<Vec<AttributeCategoryCopiedPictureBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeCategoryCopiedPictureBody {
    pub id: Option<String>,
    pub quality: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAttributeCategoryResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateAttributeCategoryError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for CreateAttributeCategoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateAttributeCategoryError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateAttributeCategoryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
