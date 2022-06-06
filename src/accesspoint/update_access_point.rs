use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateAccessPointBody {
    pub id: String,
    pub names: Option<Vec<UpdateAccessPointNameBody>>,
    pub descriptions: Option<Vec<UpdateAccessPointDescriptionBody>>,
    pub languages: Option<Vec<UpdateAccessPointLanguageBody>>,
    pub characteristics: Option<Vec<UpdateAccessPointCharacteristicBody>>,
    pub mac_address: Option<String>,
    pub logo: Option<String>,
    pub is_published_menu: Option<bool>,
    pub menu: Option<String>,
    pub default_ad: Option<AccessPointDefaultAdBody>,
    pub company_site: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAccessPointNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAccessPointDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAccessPointLanguageBody {
    pub language: Option<String>,
    pub is_main: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAccessPointCharacteristicBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessPointDefaultAdBody {
    pub id: Option<String>,
    pub url: Option<String>,
    pub pictures: Option<Vec<DefaultAdPictureBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultAdPictureBody {
    pub kind: Option<String>,
    pub picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAccessPointResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateAccessPointError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "ad_does_not_exist")]
    AdDoesNotExist,
    #[display(fmt = "access_point_not_found")]
    AccessPointNotFound,
    Default(String),
}

impl ResponseError for UpdateAccessPointError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateAccessPointError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateAccessPointError::AdDoesNotExist => {
                HttpResponse::Conflict().body("ad_does_not_exist")
            }
            UpdateAccessPointError::AccessPointNotFound => {
                HttpResponse::NotFound().body("access_point_not_found")
            }
            UpdateAccessPointError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
