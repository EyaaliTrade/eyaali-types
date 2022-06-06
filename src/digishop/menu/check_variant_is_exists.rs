use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckVariantIsExistsBody {
    pub product: String,
    pub options: Vec<OptionValueBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionValueBody {
    pub option: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckVariantIsExistsResult {
    pub is_exists: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariantIsExistsAggregation {
    pub is_exists: Option<bool>,
}

#[derive(Debug, Display)]
pub enum CheckVariantIsExistsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for CheckVariantIsExistsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CheckVariantIsExistsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CheckVariantIsExistsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
