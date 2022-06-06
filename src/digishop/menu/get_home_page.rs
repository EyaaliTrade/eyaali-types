use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHomePageBody {
    pub user: Option<String>,
    pub device: Option<String>,
    pub menu: String,
    pub language_code: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHomePageResult {
    pub slides: Option<Vec<PictureUrlAggregation>>,
    pub partner_slides: Option<Vec<PictureUrlAggregation>>,
    pub collections: Option<Vec<HomePageCollectionAggregation>>,
    pub products: Option<Vec<ProductAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomePageAggregation {
    pub slides: Option<Vec<PictureUrlAggregation>>,
    pub partner_slides: Option<Vec<PictureUrlAggregation>>,
    pub collections: Option<Vec<HomePageCollectionAggregation>>,
    pub products: Option<Vec<ProductAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomePageCollectionAggregation {
    pub position: Option<i32>,
    pub titles: Option<Vec<HomePageCollectionAggregationTitle>>,
    pub tags: Option<Vec<String>>,
    pub picture: Option<PictureUrlAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomePageCollectionAggregationTitle {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAggregation {
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

#[derive(Debug, Display)]
pub enum GetHomePageError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetHomePageError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetHomePageError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetHomePageError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
