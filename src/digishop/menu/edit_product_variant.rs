use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct EditProductVariantBody {
    pub product: String,
    pub id: String,
    pub names: Option<Vec<UpdateProductVariantNameBody>>,
    pub descriptions: Option<Vec<UpdateProductVariantDescriptionBody>>,
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
pub struct UpdateProductVariantNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductVariantDescriptionBody {
    pub id: Option<String>,
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
pub struct EditProductVariantResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum EditProductVariantError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    #[display(fmt = "variant_not_found")]
    VariantNotFound,
    Default(String),
}

impl ResponseError for EditProductVariantError {
    fn error_response(&self) -> HttpResponse {
        match self {
            EditProductVariantError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            EditProductVariantError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            EditProductVariantError::VariantNotFound => {
                HttpResponse::Conflict().body("variant_not_found")
            }
            EditProductVariantError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
