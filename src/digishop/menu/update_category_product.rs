use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateCategoryProductBody {
    pub id: String,
    pub category: Option<String>,
    pub categories: Option<Vec<String>>,
    pub identifier: Option<String>,
    pub names: Option<Vec<UpdateCategoryProductNameBody>>,
    pub descriptions: Option<Vec<UpdateCategoryProductDescriptionBody>>,
    pub kind: Option<String>,
    pub tags: Option<Vec<String>>,
    pub vendor: Option<String>,
    pub vendor_id: Option<String>,
    pub brand: Option<String>,
    pub unit: Option<CategoryProductUnitBody>,
    pub picture: Option<String>,
    pub sales_count: Option<String>,
    pub copied_pictures: Option<Vec<CategoryProductCopiedPictureBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCategoryProductNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCategoryProductDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductUnitBody {
    pub kind: Option<String>,
    pub default: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductCopiedPictureBody {
    pub id: Option<String>,
    pub quality: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCategoryProductResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateCategoryProductError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    #[display(fmt = "identifier_is_not_alphabetic")]
    IdentifierIsNotAlphabetic,
    Default(String),
}

impl ResponseError for UpdateCategoryProductError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateCategoryProductError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateCategoryProductError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            UpdateCategoryProductError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            UpdateCategoryProductError::IdentifierIsNotAlphabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabetic")
            }
            UpdateCategoryProductError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
