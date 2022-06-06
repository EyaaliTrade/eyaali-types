use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::api_data::beamandgo::ApiData;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuTypeProductBody {
    pub id: String,
    pub category: Option<String>,
    pub categories: Option<Vec<String>>,
    pub identifier: Option<String>,
    pub has_multi_languages: Option<bool>,
    pub name: Option<UpdateMenuTypeProductNameBody>,
    pub description: Option<UpdateMenuTypeProductDescriptionBody>,
    pub kind: Option<String>,
    pub tags: Option<Vec<String>>,
    pub picture: Option<String>,
    pub copied_pictures: Option<Vec<ProductCopiedPictureBody>>,
    pub price: Option<ProductPriceBody>,
    pub discount: Option<ProductDiscountBody>,
    pub attributes: Option<Vec<String>>,
    pub custom_fields: Option<Vec<ProductCustomFieldBody>>,
    pub is_published: Option<bool>,
    pub api_data: Option<ApiData>,
    pub related_to: Option<String>,
    pub is_free: Option<bool>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuTypeProductNameBody {
    pub id: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuTypeProductDescriptionBody {
    pub id: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductCopiedPictureBody {
    pub id: Option<String>,
    pub quality: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPriceBody {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductDiscountBody {
    pub percentage: Option<i32>,
    pub price: Option<ProductPriceBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductCustomFieldBody {
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuTypeProductResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuTypeProductError {
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

impl ResponseError for UpdateMenuTypeProductError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuTypeProductError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuTypeProductError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            UpdateMenuTypeProductError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            UpdateMenuTypeProductError::IdentifierIsNotAlphabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabetic")
            }
            UpdateMenuTypeProductError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
