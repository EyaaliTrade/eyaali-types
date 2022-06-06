use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductOptionsBody {
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductOptionsResult {
    pub list: Vec<ProductOptionAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductOptionAggregation {
    pub id: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
    pub descriptions: Option<Vec<DescriptionAggregation>>,
    pub input_kind: Option<String>,
    pub required_items: Option<i32>,
    pub has_default: Option<bool>,
    // pub order: Option<i32>,
    pub values: Option<Vec<ProductOptionValueAggregation>>,
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
pub struct ProductOptionValueAggregation {
    pub id: Option<String>,
    pub names: Option<Vec<ValueNameAggregation>>,
    pub descriptions: Option<Vec<ValueDescriptionAggregation>>,
    pub picture: Option<ProductPictureUrlAggregation>,
    pub price_difference: Option<f64>,
    pub is_default: Option<bool>,
    // pub order: Option<i32>,
    pub sites: Option<Vec<ProductOptionSiteAggregation>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueNameAggregation {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueDescriptionAggregation {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductOptionSiteAggregation {
    pub site: Option<String>,
    pub price_difference: Option<f64>,
}

#[derive(Debug, Display)]
pub enum GetListProductOptionsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListProductOptionsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListProductOptionsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListProductOptionsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
