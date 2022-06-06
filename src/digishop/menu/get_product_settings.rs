use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductSettingsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductSettingsResult {
    pub product_settings: ProductSettingsAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductSettingsAggregation {
    pub id: Option<String>,
    pub barcode: Option<String>,
    pub sku: Option<String>,
    pub shipping_weight: Option<f64>,
    pub price: Option<ProductPriceAggregation>,
    pub discount: Option<ProductDiscountAggregation>,
    pub has_multi_languages: Option<bool>,
    pub has_multi_sites: Option<bool>,
    pub sites: Option<Vec<CategoryProductSiteAggregation>>,
    pub has_variants: Option<bool>,
    pub is_published: Option<bool>,
    pub must_check_availaibitly: Option<bool>,
    pub delivery_mode: Option<String>
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
pub struct CategoryProductSiteAggregation {
    pub site: Option<String>,
    pub sku: Option<String>,
    pub price: Option<ProductPriceAggregation>,
    pub discount: Option<ProductDiscountAggregation>,
}

#[derive(Debug, Display)]
pub enum GetProductSettingsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for GetProductSettingsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductSettingsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductSettingsError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            GetProductSettingsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
