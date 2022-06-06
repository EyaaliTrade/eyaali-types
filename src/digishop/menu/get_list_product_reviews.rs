use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductReviewsBody {
    pub user: Option<String>,
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductReviewsResult {
    pub list: Vec<ProductReviewAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductReviewAggregation {
    pub id: Option<String>,
    pub is_me: Option<bool>,
    pub user: Option<String>,
    pub rating: Option<i32>,
    pub comment: Option<String>,
    pub pictures: Option<Vec<PictureUrlAggregation>>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListProductReviewsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListProductReviewsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListProductReviewsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListProductReviewsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
