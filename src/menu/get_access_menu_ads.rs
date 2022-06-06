use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessMenuAdsBody {
    pub menu: String,
    pub position: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessMenuAdsResult {
    pub list: Vec<AccessMenuAdAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessMenuAdAggregation {
    pub id: Option<String>,
    pub names: Option<Vec<MenuAdNameAggregation>>,
    pub descriptions: Option<Vec<MenuAdDescriptionAggregation>>,
    pub kind: Option<String>,
    pub position: Option<String>,
    pub pictures: Option<MenuAdPictureAggregation>,
    pub products: Option<Vec<ProductIdAggregation>>,
    pub external_url: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
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
pub enum GetAccessMenuAdsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetAccessMenuAdsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAccessMenuAdsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetAccessMenuAdsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
