use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductSpecsBody {
    pub product: String,
    pub identifier: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductSpecsResult {
    pub list: Vec<ProductSpecAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductSpecAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub title: Option<String>,
    pub specifications: Option<Vec<SpecificationAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpecificationAggregation {
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListProductSpecsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListProductSpecsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListProductSpecsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListProductSpecsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
