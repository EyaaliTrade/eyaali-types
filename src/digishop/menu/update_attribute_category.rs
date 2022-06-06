use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateAttributeCategoryBody {
    pub id: String,
    pub name: Option<UpdateAttributeCategoryNameBody>,
    pub picture: Option<String>,
    pub copied_pictures: Option<Vec<AttributeCategoryCopiedPictureBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAttributeCategoryNameBody {
    pub id: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeCategoryCopiedPictureBody {
    pub id: Option<String>,
    pub quality: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAttributeCategoryResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateAttributeCategoryError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "attribute_category_not_found")]
    AttributeCategoryNotFound,
    Default(String),
}

impl ResponseError for UpdateAttributeCategoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateAttributeCategoryError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateAttributeCategoryError::AttributeCategoryNotFound => {
                HttpResponse::Conflict().body("attribute_category_not_found")
            }
            UpdateAttributeCategoryError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
