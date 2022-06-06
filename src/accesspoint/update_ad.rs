use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateAdBody {
    pub id: String,
    pub titles: Option<Vec<UpdateAdTitleBody>>,
    pub descriptions: Option<Vec<UpdateDescriptionBody>>,
    pub url: Option<String>,
    pub is_published: Option<bool>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub pictures: Option<Vec<UpdateAdPictureBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAdTitleBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAdPictureBody {
    pub kind: Option<String>,
    pub picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAdResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateAdError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "ad_not_found")]
    AdNotFound,
    Default(String),
}

impl ResponseError for UpdateAdError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateAdError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateAdError::AdNotFound => HttpResponse::NotFound().body("access_point_not_found"),
            UpdateAdError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
