use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SetHomePageBody {
    pub menu: String,
    pub slides: Option<Vec<HomePageSlideBody>>,
    pub partner_slides: Option<Vec<HomePageSlideBody>>,
    pub collections: Option<Vec<HomePageCollectionBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomePageSlideBody {
    pub picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomePageCollectionBody {
    pub position: Option<i32>,
    pub titles: Option<Vec<SetHomePageCollectionTitleBody>>,
    pub tags: Option<Vec<String>>,
    pub picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetHomePageCollectionTitleBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetHomePageResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum SetHomePageError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for SetHomePageError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SetHomePageError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SetHomePageError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
