use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryProductDetailsBody {
    pub id: String,
    pub language_code: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryProductDetailsResult {
    pub product: CategoryProductDetailsAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductDetailsAggregation {
    pub id: Option<String>,
    pub category: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
    pub descriptions: Option<Vec<DescriptionAggregation>>,
    pub kind: Option<String>,
    pub tags: Option<Vec<String>>,
    pub vendor: Option<String>,
    pub vendor_id: Option<String>,
    pub brand: Option<String>,
    pub sales_count: Option<String>,
    pub unit: Option<ProductUnitAggregation>,
    pub picture: Option<ProductPictureUrlAggregation>,
    pub gallery: Option<Vec<ProductPictureUrlAggregation>>,
    pub related_to: Option<String>,
    pub is_free: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductUnitAggregation {
    pub kind: Option<String>,
    pub default: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceCurrencyAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetCategoryProductDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for GetCategoryProductDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCategoryProductDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCategoryProductDetailsError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            GetCategoryProductDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
