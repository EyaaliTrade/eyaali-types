use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuProductsBody {
    pub menu: String,
    pub vendor_id: Option<String>,
    pub filter_category: Option<String>,
    pub current_category: Option<String>,
    pub name: Option<String>,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub brand: Option<String>,
    pub sorting: Option<String>,
    pub language_code: Option<String>,
    pub page_number: Option<i32>,
    pub page_count: Option<i32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuProductsResult {
    pub list: Vec<ProductToAssignAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductToAssignAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub brand: Option<String>,
    pub price: Option<ProductPriceAggregation>,
    pub picture: Option<CategoryPictureUrlAggregation>,
    pub main_category: Option<CategoryToAssignAggregation>,
    pub current_category: Option<CategoryToAssignAggregation>,
    pub must_check_availaibitly: Option<bool>,
    pub delivery_mode: Option<String>,
    pub sales_count: Option<String>,
    pub options_count: Option<i32>,
    pub variants_count: Option<i32>,
    pub is_assigned: Option<bool>,
    pub has_multi_languages: Option<bool>,
    pub is_free: Option<bool>,
    pub related_to: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
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
pub struct CategoryPictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryToAssignAggregation {
    pub id: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
    pub level: Option<i32>,
}

#[derive(Debug, Display)]
pub enum GetListMenuProductsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListMenuProductsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMenuProductsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListMenuProductsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
