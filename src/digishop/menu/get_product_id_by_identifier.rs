use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductIdByIdentifierBody {
    pub identifier: String,
    pub menu: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductIdAggregation {
    pub id: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductIdByIdentifierResult {
    pub id: String,
}


#[derive(Debug, Display)]
pub enum GetProductIdByIdentifierError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for GetProductIdByIdentifierError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductIdByIdentifierError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductIdByIdentifierError::ProductNotFound => {
                HttpResponse::NotFound().body("product_not_found")
            }
            GetProductIdByIdentifierError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}