use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFilterCollectionDetailsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFilterCollectionDetailsResult {
    pub id: Option<String>,
    pub menu: Option<String>,
    pub names: Option<Vec<FilterCollectionNameAggregation>>,
    pub descriptions: Option<Vec<FilterCollectionDescriptionAggregation>>,
    pub categories: Option<Vec<FilterCollectionCategoryIdAggregation>>,
    pub products: Option<Vec<FilterCollectionProductIdAggregation>>,
    pub options: Option<Vec<FilterCollectionOptionAggregation>>,
    pub max_items: Option<i32>,
    pub sorting: Option<String>,
    pub price_range: Option<FilterCollectionPriceRangeAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterCollectionNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterCollectionDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterCollectionCategoryIdAggregation {
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterCollectionProductIdAggregation {
    pub product: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterCollectionOptionAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterCollectionPriceRangeAggregation {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Display)]
pub enum GetFilterCollectionDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetFilterCollectionDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetFilterCollectionDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetFilterCollectionDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
