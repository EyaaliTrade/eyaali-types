use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetAccessPointByIdBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessPointByIdResult {
    pub access_point: AccessPointAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessPointAggregation {
    pub id: Option<String>,
    pub names: Option<Vec<AccessPointNameAggregation>>,
    pub descriptions: Option<Vec<AccessPointDescriptionAggregation>>,
    pub languages: Option<Vec<AccessPointLanguageAggregation>>,
    pub characteristics: Option<Vec<AccessPointCharacteristicAggregation>>,
    pub mac_address: Option<String>,
    pub is_published_menu: Option<bool>,
    pub menu: Option<String>,
    pub default_ad: Option<AccessPointDefaultAdAggregation>,
    pub logo: Option<PictureAggregation>,
    pub company_site: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessPointNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessPointDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessPointLanguageAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub is_main: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessPointCharacteristicAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessPointDefaultAdAggregation {
    pub id: Option<String>,
    pub url: Option<String>,
    pub pictures: Option<Vec<AdPictureAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdPictureAggregation {
    pub kind: Option<String>,
    pub picture: Option<PictureAggregation>,
}

#[derive(Debug, Display)]
pub enum GetAccessPointByIdError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "access_point_not_found")]
    AccessPointNotFound,
    Default(String),
}

impl ResponseError for GetAccessPointByIdError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAccessPointByIdError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetAccessPointByIdError::AccessPointNotFound => {
                HttpResponse::NotFound().body("access_point_not_found")
            }
            GetAccessPointByIdError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
