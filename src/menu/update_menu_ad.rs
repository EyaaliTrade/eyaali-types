use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuAdBody {
    pub id: String,
    pub number_of_authorized_ads: i32,
    pub number_of_published_ads: i32,
    pub menus: Option<Vec<MenuIdBody>>,
    pub names: Option<Vec<UpdateMenuAdNameBody>>,
    pub descriptions: Option<Vec<UpdateMenuAdDescriptionBody>>,
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
pub struct UpdateMenuAdNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuAdDescriptionBody {
    pub id: Option<String>,
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
pub struct UpdateMenuAdResult {
    pub success: bool,
    pub is_reached: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuAdError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_ad_not_found")]
    MenuAdNotFound,
    #[display(fmt = "maximum_published_ad_reached")]
    MaximumReached,
    Default(String),
}

impl ResponseError for UpdateMenuAdError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuAdError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuAdError::MenuAdNotFound => HttpResponse::Conflict().body("menu_ad_not_found"),
            UpdateMenuAdError::MaximumReached => {
                HttpResponse::Gone().body("maximum_published_ad_reached")
            }
            UpdateMenuAdError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
