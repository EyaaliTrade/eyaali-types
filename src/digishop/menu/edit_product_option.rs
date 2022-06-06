use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct EditProductOptionBody {
    pub product: String,
    pub id: String,
    pub names: Option<Vec<UpdateProductOptionNameBody>>,
    pub descriptions: Option<Vec<UpdateProductOptionDescriptionBody>>,
    pub input_kind: Option<String>,
    pub required_items: Option<i32>,
    pub has_default: Option<bool>,
    pub values: Option<Vec<ProductOptionValueBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductOptionNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductOptionDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductOptionValueBody {
    pub id: Option<String>,
    pub names: Option<Vec<UpdateProductOptionNameBody>>,
    pub descriptions: Option<Vec<UpdateProductOptionDescriptionBody>>,
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
pub struct EditProductOptionResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum EditProductOptionError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    #[display(fmt = "option_not_found")]
    OptionNotFound,
    Default(String),
}

impl ResponseError for EditProductOptionError {
    fn error_response(&self) -> HttpResponse {
        match self {
            EditProductOptionError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            EditProductOptionError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            EditProductOptionError::OptionNotFound => {
                HttpResponse::Conflict().body("option_not_found")
            }
            EditProductOptionError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
