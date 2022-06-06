use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductRatingCountsBody {
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductRatingCountsResult {
    pub rating_counts: Vec<RatingCountAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RatingCountAggregation {
    pub value: Option<i32>,
    pub count: Option<i32>,
}

#[derive(Debug, Display)]
pub enum GetProductRatingCountsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductRatingCountsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductRatingCountsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductRatingCountsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
