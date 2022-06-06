use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use crate::api_data::beamandgo::ApiData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuTypeProductsBody {
    pub menu_type: String,
    pub name: Option<String>,
    pub sorting: Option<String>,
    pub is_published: Option<bool>,
    pub category: Option<String>,
    pub page_number: i32,
    pub page_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuTypeProductsResult {
    pub total: i32,
    pub list: Vec<MenuTypeProductAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuTypeProductAggregation {
    pub id: Option<String>,
    pub category: Option<CategoryAggregation>,
    pub categories: Option<Vec<CategoryAggregation>>,
    pub identifier: Option<String>,
    pub has_multi_languages: Option<bool>,
    pub name: Option<NameAggregation>,
    pub description: Option<DescriptionAggregation>,
    pub kind: Option<String>,
    pub tags: Option<Vec<String>>,
    pub picture: Option<PictureUrlAggregation>,
    pub gallery: Option<Vec<PictureUrlAggregation>>,
    pub price: Option<ProductPriceAggregation>,
    pub discount: Option<ProductDiscountAggregation>,
    pub attributes: Option<Vec<ProductAttributeAggregation>>,
    pub custom_fields: Option<Vec<ProductCustomFieldAggregation>>,
    pub is_published: Option<bool>,
    pub created_at: Option<String>,
    pub api_data: Option<ApiData>,
    pub related_to: Option<String>,
    pub is_free: Option<bool>

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryAggregation {
    pub id: Option<String>,
    pub name: Option<NameAggregation>,
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
pub struct PictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPriceAggregation {
    pub value: Option<f64>,
    pub currency: Option<PriceCurrencyAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceCurrencyAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductDiscountAggregation {
    pub percentage: Option<i32>,
    pub price: Option<ProductPriceAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAttributeAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub name: Option<NameAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductCustomFieldAggregation {
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetMenuTypeProductsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetMenuTypeProductsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuTypeProductsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuTypeProductsError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetMenuTypeProductsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
