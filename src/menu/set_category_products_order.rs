use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SetCategoryProductsOrderBody {
    pub products: Option<Vec<CategoryProductOrderBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductOrderBody {
    pub id: String,
    pub order: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetCategoryProductsOrderResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum SetCategoryProductsOrderError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for SetCategoryProductsOrderError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SetCategoryProductsOrderError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SetCategoryProductsOrderError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
