use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductsBody {
    pub user: Option<String>,
    pub device: Option<String>,
    pub menu: String,
    pub main_category: Option<String>,
    pub categories: Option<Vec<String>>,
    pub products: Option<Vec<String>>,
    pub name: Option<String>,
    pub vendors: Option<Vec<String>>,
    pub brands: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub in_discount: Option<bool>,
    pub availability: Option<String>,
    pub price: Option<PriceFilterBody>,
    pub sorting: Option<String>,
    pub page_number: i32,
    pub page_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceFilterBody {
    pub min: f64,
    pub max: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductsResult {
    pub list: Vec<ProductAggregation>,
    pub total: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
    pub descriptions: Option<Vec<DescriptionAggregation>>,
    pub vendor_id: Option<String>,
    pub price: Option<ProductPriceAggregation>,
    pub discount: Option<ProductDiscountAggregation>,
    pub picture: Option<PictureUrlAggregation>,
    pub copied_pictures: Option<Vec<PictureAggregation>>,
    pub attributes: Option<Vec<ProductAttributeAggregation>>,
    pub product_specs: Option<Vec<ProductSpecAggregation>>,
    pub has_options: Option<bool>,
    pub must_check_availaibitly: Option<bool>,
    pub is_favorite: Option<bool>,
    pub created_at: Option<String>,
    pub related_to: Option<String>,
    pub is_free: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductSpecAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub title: Option<String>,
    pub specifications: Option<Vec<KeyValueAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyValueAggregation {
    pub key: Option<String>,
    pub value: Option<String>,
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
pub struct PictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAttributeAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub name: Option<NameAggregation>,
}

#[derive(Debug, Display)]
pub enum GetListProductsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetListProductsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListProductsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListProductsError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetListProductsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
