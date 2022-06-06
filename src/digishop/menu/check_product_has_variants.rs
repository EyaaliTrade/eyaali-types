use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckProductHasVariantsBody {
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckProductHasVariantsResult {
    pub has_variants: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductHasVariantsAggregation {
    pub has_variants: Option<bool>,
}

#[derive(Debug, Display)]
pub enum CheckProductHasVariantsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for CheckProductHasVariantsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CheckProductHasVariantsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CheckProductHasVariantsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
