use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateFilterCollectionBody {
    pub menu: Option<String>,
    pub names: Option<Vec<CreateFilterCollectionNameBody>>,
    pub descriptions: Option<Vec<CreateFilterCollectionDescriptionBody>>,
    pub categories: Option<Vec<CategoryIdBody>>,
    pub products: Option<Vec<ProductIdBody>>,
    pub options: Option<Vec<FilterCollectionOptionBody>>,
    pub max_items: Option<i32>,
    pub sorting: Option<String>,
    pub price_range: Option<FilterCollectionPriceRangeBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateFilterCollectionNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateFilterCollectionDescriptionBody {
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
pub struct CreateFilterCollectionResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateFilterCollectionError {
    Default(String),
}

impl ResponseError for CreateFilterCollectionError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateFilterCollectionError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
