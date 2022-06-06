use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductAverageRatingBody {
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductAverageRatingResult {
    pub average_rating: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAverageRatingAggregation {
    pub average_rating: Option<f64>,
}

#[derive(Debug, Display)]
pub enum GetProductAverageRatingError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductAverageRatingError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductAverageRatingError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductAverageRatingError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
