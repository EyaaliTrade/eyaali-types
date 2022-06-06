use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct EditProductSettingsBody {
    pub id: String,
    pub barcode: Option<String>,
    pub sku: Option<String>,
    pub shipping_weight: Option<f64>,
    pub price: Option<CategoryProductPriceBody>,
    pub discount: Option<CategoryProductDiscountBody>,
    pub has_multi_languages: Option<bool>,
    pub has_multi_sites: Option<bool>,
    pub sites: Option<Vec<CategoryProductSiteBody>>,
    pub has_variants: Option<bool>,
    pub must_check_availaibitly: Option<bool>,
    pub delivery_mode: Option<String>,
    pub is_published: Option<bool>,
    pub is_free: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedeemPoints {
    pub points: i32,
    pub cash: f64,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductPriceBody {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductDiscountBody {
    pub percentage: Option<i32>,
    pub price: Option<CategoryProductPriceBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductSiteBody {
    pub site: Option<String>,
    pub sku: Option<String>,
    pub price: Option<CategoryProductPriceBody>,
    pub discount: Option<CategoryProductDiscountBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditProductSettingsResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum EditProductSettingsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for EditProductSettingsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            EditProductSettingsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            EditProductSettingsError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            EditProductSettingsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
