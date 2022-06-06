use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateCategoryProductBody {
    pub id: String,
    pub category: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<UpdateCategoryProductNameBody>>,
    pub short_descriptions: Option<Vec<UpdateCategoryProductDescriptionBody>>,
    pub long_descriptions: Option<Vec<UpdateCategoryProductDescriptionBody>>,
    pub price: Option<CategoryProductPriceBody>,
    pub discount: Option<CategoryProductDiscountBody>,
    pub unit: Option<CategoryProductUnitBody>,
    pub picture: Option<String>,
    pub copied_pictures: Option<Vec<CategoryProductCopiedPictureBody>>,
    pub order: Option<i32>,
    pub is_published: Option<bool>,
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
    #[display(fmt = "category_product_not_found")]
    CategoryProductNotFound,
    #[display(fmt = "identifier_is_not_alphabectic")]
    IdentifierIsNotAplhabetic,
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    Default(String),
}

impl ResponseError for UpdateCategoryProductError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateCategoryProductError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateCategoryProductError::CategoryProductNotFound => {
                HttpResponse::Conflict().body("category_product_not_found")
            }
            UpdateCategoryProductError::IdentifierIsNotAplhabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabectic")
            }
            UpdateCategoryProductError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            UpdateCategoryProductError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
