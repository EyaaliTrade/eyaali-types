use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFilterCollectionCharacteristicsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFilterCollectionCharacteristicsResult {
    pub categories: Vec<String>,
    pub products: Vec<String>,
    pub max_items: Option<i32>,
    pub sorting: Option<String>,
    pub min_price_range: Option<f64>,
    pub max_price_range: Option<f64>,
}

#[derive(Debug, Display)]
pub enum GetFilterCollectionCharacteristicsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetFilterCollectionCharacteristicsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetFilterCollectionCharacteristicsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetFilterCollectionCharacteristicsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
