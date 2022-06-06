use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AddProductOptionBody {
    pub product: String,
    pub names: Option<Vec<CreateProductOptionNameBody>>,
    pub descriptions: Option<Vec<CreateProductOptionDescriptionBody>>,
    pub input_kind: Option<String>,
    pub required_items: Option<i32>,
    pub has_default: Option<bool>,
    pub values: Option<Vec<ProductOptionValueBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProductOptionNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProductOptionDescriptionBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductOptionValueBody {
    pub names: Option<Vec<CreateProductOptionNameBody>>,
    pub descriptions: Option<Vec<CreateProductOptionDescriptionBody>>,
    pub picture: Option<String>,
    pub copied_pictures: Option<Vec<CategoryProductCopiedPictureBody>>,
    pub price_difference: Option<f64>,
    pub is_default: Option<bool>,
    pub sites: Option<Vec<ProductOptionSiteBody>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductCopiedPictureBody {
    pub id: Option<String>,
    pub quality: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductOptionSiteBody {
    pub site: Option<String>,
    pub price_difference: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddProductOptionResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum AddProductOptionError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for AddProductOptionError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AddProductOptionError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            AddProductOptionError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            AddProductOptionError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
