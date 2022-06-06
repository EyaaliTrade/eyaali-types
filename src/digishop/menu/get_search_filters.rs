use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSearchFiltersBody {
    pub menu: String,
    pub language_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSearchFiltersResult {
    pub filters: FiltersAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FiltersAggregation {
    pub price_range: Option<PriceRangeAggregation>,
    pub vendors: Option<Vec<String>>,
    pub brands: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<CategoryFilterAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceRangeAggregation {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoriesFilterAggregation {
    pub categories: Vec<CategoryFilterAggregation>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryFilterAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrandsAggregation {
    pub brands: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VendorsAggregation {
    pub vendors: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagsAggregation {
    pub tags: Vec<String>,
}

#[derive(Debug, Display)]
pub enum GetSearchFiltersError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetSearchFiltersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetSearchFiltersError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetSearchFiltersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}