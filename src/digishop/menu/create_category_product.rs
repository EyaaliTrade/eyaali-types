use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateCategoryProductBody {
    pub menu: String,
    pub category: Option<String>,
    pub categories: Option<Vec<String>>,
    pub identifier: Option<String>,
    pub names: Option<Vec<CreateCategoryProductNameBody>>,
    pub descriptions: Option<Vec<CreateCategoryProductDescriptionBody>>,
    pub kind: Option<String>,
    pub tags: Option<Vec<String>>,
    pub vendor: Option<String>,
    pub vendor_id: Option<String>,
    pub brand: Option<String>,
    pub unit: Option<CategoryProductUnitBody>,
    pub picture: Option<String>,
    pub copied_pictures: Option<Vec<CategoryProductCopiedPictureBody>>,
    pub sales_count: Option<String>,
    pub gallery: Option<Vec<CategoryProductGalleryBody>>,
    pub is_free: Option<bool>

}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCategoryProductNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCategoryProductDescriptionBody {
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
pub struct CategoryProductGalleryBody {
    pub picture: Option<String>,
    pub copied_pictures: Option<Vec<CategoryProductCopiedPictureBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCategoryProductResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateCategoryProductError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    #[display(fmt = "identifier_is_not_alphabetic")]
    IdentifierIsNotAlphabetic,
    Default(String),
}

impl ResponseError for CreateCategoryProductError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateCategoryProductError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateCategoryProductError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            CreateCategoryProductError::IdentifierIsNotAlphabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabetic")
            }
            CreateCategoryProductError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
