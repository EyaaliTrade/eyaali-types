use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductDetailsBody {
    pub user: Option<String>,
    pub device: Option<String>,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductDetailsResult {
    pub product: ProductDetailsAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductDetailsAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
    pub descriptions: Option<Vec<DescriptionAggregation>>,
    pub kind: Option<String>,
    pub tags: Option<Vec<String>>,
    pub vendor: Option<String>,
    pub vendor_id: Option<String>,
    pub brand: Option<String>,
    pub sku: Option<String>,
    pub unit: Option<ProductUnitAggregation>,
    pub price: Option<ProductPriceAggregation>,
    pub discount: Option<ProductDiscountAggregation>,
    pub attributes: Option<Vec<ProductAttributeAggregation>>,
    pub product_specs: Option<Vec<ProductSpecAggregation>>,
    pub has_variants: Option<bool>,
    pub is_favorite: Option<bool>,
    pub must_check_availaibitly: Option<bool>,
    pub delivery_mode: Option<String>,
    pub sales_count: Option<String>,
    pub is_free: Option<bool>
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
pub struct ProductPriceAggregation {
    pub value: Option<f64>,
    pub currency: Option<PriceCurrencyAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceCurrencyAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductDiscountAggregation {
    pub percentage: Option<i32>,
    pub price: Option<ProductPriceAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAttributeAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub name: Option<NameAggregation>,
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
pub enum GetProductDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for GetProductDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductDetailsError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetProductDetailsError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            GetProductDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
