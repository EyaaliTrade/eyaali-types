use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AddProductVariantBody {
    pub product: String,
    pub names: Option<Vec<CreateProductVariantNameBody>>,
    pub descriptions: Option<Vec<CreateProductVariantDescriptionBody>>,
    pub barcode: Option<String>,
    pub sku: Option<String>,
    pub shipping_weight: Option<f64>,
    pub picture: Option<String>,
    pub copied_pictures: Option<Vec<CategoryProductCopiedPictureBody>>,
    pub price_difference: Option<f64>,
    pub discount_percentage: Option<i32>,
    pub sites: Option<Vec<ProductVariantSiteBody>>,
    pub options: Option<Vec<ProductVariantOptionBody>>,
    pub is_published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProductVariantNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProductVariantDescriptionBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductCopiedPictureBody {
    pub id: Option<String>,
    pub quality: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductVariantSiteBody {
    pub site: Option<String>,
    pub sku: Option<String>,
    pub price_difference: Option<f64>,
    pub discount_percentage: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductVariantOptionBody {
    pub option: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddProductVariantResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum AddProductVariantError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for AddProductVariantError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AddProductVariantError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            AddProductVariantError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            AddProductVariantError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
