use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductVariantsBody {
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductVariantsResult {
    pub list: Vec<ProductVariantAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductVariantAggregation {
    pub id: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
    pub descriptions: Option<Vec<DescriptionAggregation>>,
    pub barcode: Option<String>,
    pub sku: Option<String>,
    pub shipping_weight: Option<f64>,
    pub picture: Option<ProductPictureUrlAggregation>,
    pub price_difference: Option<f64>,
    pub discount_percentage: Option<i32>,
    pub sites: Option<Vec<ProductVariantSiteAggregation>>,
    pub options: Option<Vec<ProductVariantOptionAggregation>>,
    pub is_published: Option<bool>,
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
pub struct ProductPictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductVariantSiteAggregation {
    pub site: Option<String>,
    pub sku: Option<String>,
    pub price_difference: Option<f64>,
    pub discount_percentage: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductVariantOptionAggregation {
    pub option: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListProductVariantsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListProductVariantsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListProductVariantsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListProductVariantsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
