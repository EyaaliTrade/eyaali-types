use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFilterCollectionProductsBody {
    pub id: String,
    pub language_code: String,
    pub products: Vec<String>,
    pub max_items: i32,
    pub sorting: String,
    pub min_price_range: f64,
    pub max_price_range: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFilterCollectionProductsResult {
    pub products: Option<Vec<ProductIdAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductIdAggregation {
    pub id: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetFilterCollectionProductsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetFilterCollectionProductsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetFilterCollectionProductsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetFilterCollectionProductsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
