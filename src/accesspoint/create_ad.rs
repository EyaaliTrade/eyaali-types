use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateAdBody {
    pub titles: Option<Vec<CreateAdTitleBody>>,
    pub descriptions: Option<Vec<CreateDescriptionBody>>,
    pub url: Option<String>,
    pub is_published: Option<bool>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub pictures: Option<Vec<CreateAdPictureBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAdTitleBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateDescriptionBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAdPictureBody {
    pub kind: Option<String>,
    pub picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAdResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateAdError {
    Default(String),
}

impl ResponseError for CreateAdError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateAdError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
