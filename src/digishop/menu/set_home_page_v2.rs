use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SetHomePageV2Body {
    pub menu: String,
    pub slides: Option<Vec<HomePageV2SlideBody>>,
    pub collections: Option<Vec<HomePageV2CollectionBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomePageV2SlideBody {
    pub picture: Option<String>,
    pub kind: Option<String>,
    pub object: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomePageV2CollectionBody {
    pub position: Option<i32>,
    pub titles: Option<Vec<SetHomePageV2CollectionTitleBody>>,
    pub tags: Option<Vec<String>>,
    pub picture: Option<String>,
    pub kind: Option<String>,
    pub object: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetHomePageV2CollectionTitleBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetHomePageV2Result {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum SetHomePageV2Error {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for SetHomePageV2Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            SetHomePageV2Error::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SetHomePageV2Error::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
