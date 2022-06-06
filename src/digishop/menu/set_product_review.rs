use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SetProductReviewBody {
    pub user: String,
    pub product: String,
    pub rating: i32,
    pub comment: Option<String>,
    pub pictures: Option<Vec<ProductReviewPictureBody>>,
    pub is_allowed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductReviewPictureBody {
    pub picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetProductReviewResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum SetProductReviewError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    #[display(fmt = "action_not_allowed")]
    ActionNotAllowed,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for SetProductReviewError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SetProductReviewError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SetProductReviewError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            SetProductReviewError::ActionNotAllowed => {
                HttpResponse::Forbidden().body("action_not_allowed")
            }
            SetProductReviewError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            SetProductReviewError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
