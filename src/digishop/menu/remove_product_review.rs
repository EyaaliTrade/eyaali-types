use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RemoveProductReviewBody {
    pub user: String,
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveProductReviewResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum RemoveProductReviewError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    #[display(fmt = "review_not_found")]
    ReviewNotFound,
    Default(String),
}

impl ResponseError for RemoveProductReviewError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RemoveProductReviewError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            RemoveProductReviewError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            RemoveProductReviewError::ReviewNotFound => {
                HttpResponse::Conflict().body("review_not_found")
            }
            RemoveProductReviewError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
