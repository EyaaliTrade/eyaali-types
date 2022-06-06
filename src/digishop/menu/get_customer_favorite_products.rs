use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCustomerFavoriteProductsBody {
    pub user: Option<String>,
    pub device: Option<String>,
    pub menu: String,
    pub page_number: i32,
    pub page_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCustomerFavoriteProductsResult {
    pub list: Vec<FavoriteProductAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FavoriteProductAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
    pub descriptions: Option<Vec<DescriptionAggregation>>,
    pub price: Option<ProductPriceAggregation>,
    pub discount: Option<ProductDiscountAggregation>,
    pub picture: Option<PictureUrlAggregation>,
    pub has_options: Option<bool>,
    pub is_favorite: Option<bool>,
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

#[derive(Debug, Display)]
pub enum GetCustomerFavoriteProductsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetCustomerFavoriteProductsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCustomerFavoriteProductsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCustomerFavoriteProductsError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetCustomerFavoriteProductsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
