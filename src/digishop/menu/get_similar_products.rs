use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSimilarProductsBody {
    pub product: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSimilarProductsResult {
    pub list: Vec<SimilarProductAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimilarProductAggregation {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductSimilarAggregation {
    pub id: String,
    pub count: Option<i32>,
    pub is_random: Option<bool>,
    pub category: Option<String>,
    pub products: Option<Vec<String>>
}

#[derive(Debug, Display)]
pub enum GetSimilarProductsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for GetSimilarProductsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetSimilarProductsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetSimilarProductsError::ProductNotFound => {
                HttpResponse::NotFound().body("product_not_found")
            }
            GetSimilarProductsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
