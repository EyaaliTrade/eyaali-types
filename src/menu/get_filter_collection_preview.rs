use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFilterCollectionPreviewBody {
    pub menu: String,
    pub language_code: String,
    pub categories: Option<Vec<CategoryIdBody>>,
    pub products: Option<Vec<ProductIdBody>>,
    pub options: Option<Vec<FilterCollectionOptionBody>>,
    pub max_items: Option<i32>,
    pub sorting: Option<String>,
    pub price_range: Option<FilterCollectionPriceRangeBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryIdBody {
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductIdBody {
    pub product: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterCollectionOptionBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterCollectionPriceRangeBody {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFilterCollectionPreviewResult {
    pub products: Option<Vec<ProductIdAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductIdAggregation {
    pub id: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetFilterCollectionPreviewError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetFilterCollectionPreviewError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetFilterCollectionPreviewError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetFilterCollectionPreviewError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
