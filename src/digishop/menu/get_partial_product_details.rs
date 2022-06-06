use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPartialProductDetailsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPartialProductDetailsResult {
    pub product: Option<PartialProductDetailsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialProductDetailsAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
    pub descriptions: Option<Vec<DescriptionAggregation>>,
    pub unit: Option<ProductUnitAggregation>,
    pub picture: Option<ProductPictureUrlAggregation>,
    pub copied_pictures: Option<Vec<PictureAggregation>>,
    pub delivery_mode: Option<String>
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
pub struct ProductUnitAggregation {
    pub kind: Option<String>,
    pub default: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PictureAggregation {
    pub id: Option<String>,
    pub file_name: Option<String>,
    pub kind: Option<String>,
    pub quality: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetPartialProductDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for GetPartialProductDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetPartialProductDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetPartialProductDetailsError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            GetPartialProductDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
