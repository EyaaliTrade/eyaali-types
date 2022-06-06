use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetProductSeoDataBody {
    pub id: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductSeoDataResult {
    pub product: ProductSeoAggregation,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductSeoAggregation {
    pub product: Option<String>,
    pub identifier: Option<String>,
    pub titles: Option<Vec<NameAggregation>>,
    pub descriptions: Option<Vec<DescriptionAggregation>>
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

#[derive(Debug, Display)]
pub enum GetProductSeoDataError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for GetProductSeoDataError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductSeoDataError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductSeoDataError::ProductNotFound => {
                HttpResponse::NotFound().body("product_not_found")
            }
            GetProductSeoDataError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
