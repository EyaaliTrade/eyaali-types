use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuAdsBody {
    pub company: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuAdsResult {
    pub list: Vec<MenuAdAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuAdAggregation {
    pub id: Option<String>,
    pub menus: Option<Vec<MenuIdAggregation>>,
    pub names: Option<Vec<MenuAdNameAggregation>>,
    pub descriptions: Option<Vec<MenuAdDescriptionAggregation>>,
    pub kind: Option<String>,
    pub position: Option<String>,
    pub pictures: Option<MenuAdPictureAggregation>,
    pub products: Option<Vec<ProductIdAggregation>>,
    pub external_url: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub is_published: Option<bool>,
    pub ad_clicks_count: Option<i32>,
    pub url_clicks_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuIdAggregation {
    pub menu: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuAdNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuAdDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuAdPictureAggregation {
    pub cover: Option<PictureAggregation>,
    pub poster: Option<PictureAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductIdAggregation {
    pub product: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListMenuAdsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListMenuAdsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMenuAdsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListMenuAdsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
