use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateFilterCollectionBody {
    pub id: String,
    pub names: Option<Vec<UpdateFilterCollectionNameBody>>,
    pub descriptions: Option<Vec<UpdateFilterCollectionDescriptionBody>>,
    pub categories: Option<Vec<CategoryIdBody>>,
    pub products: Option<Vec<ProductIdBody>>,
    pub options: Option<Vec<FilterCollectionOptionBody>>,
    pub max_items: Option<i32>,
    pub sorting: Option<String>,
    pub price_range: Option<FilterCollectionPriceRangeBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateFilterCollectionNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateFilterCollectionDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
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
pub struct UpdateFilterCollectionResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateFilterCollectionError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "filter_collection_not_found")]
    FilterCollectionNotFound,
    Default(String),
}

impl ResponseError for UpdateFilterCollectionError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateFilterCollectionError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateFilterCollectionError::FilterCollectionNotFound => {
                HttpResponse::Conflict().body("filter_collection_not_found")
            }
            UpdateFilterCollectionError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
