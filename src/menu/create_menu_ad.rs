use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateMenuAdBody {
    pub number_of_authorized_ads: i32,
    pub number_of_published_ads: i32,
    pub company: Option<String>,
    pub menus: Option<Vec<MenuIdBody>>,
    pub names: Option<Vec<CreateMenuAdNameBody>>,
    pub descriptions: Option<Vec<CreateMenuAdDescriptionBody>>,
    pub kind: Option<String>,
    pub position: Option<String>,
    pub pictures: Option<MenuAdPictureBody>,
    pub products: Option<Vec<ProductIdBody>>,
    pub external_url: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuIdBody {
    pub menu: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuAdNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuAdDescriptionBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuAdPictureBody {
    pub cover: Option<String>,
    pub poster: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductIdBody {
    pub product: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuAdResult {
    pub id: String,
    pub is_reached: bool,
}

#[derive(Debug, Display)]
pub enum CreateMenuAdError {
    #[display(fmt = "maximum_published_ad_reached")]
    MaximumReached,
    Default(String),
}

impl ResponseError for CreateMenuAdError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateMenuAdError::MaximumReached => {
                HttpResponse::Gone().body("maximum_published_ad_reached")
            }
            CreateMenuAdError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
