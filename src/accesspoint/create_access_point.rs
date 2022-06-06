use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateAccessPointBody {
    pub company: Option<String>,
    pub names: Option<Vec<CreateAccessPointNameBody>>,
    pub descriptions: Option<Vec<CreateAccesssPointDescriptionBody>>,
    pub languages: Option<Vec<CreateAccessPointLanguageBody>>,
    pub characteristics: Option<Vec<AccessPointCharacteristicBody>>,
    pub mac_address: String,
    pub logo: Option<String>,
    pub is_published_menu: Option<bool>,
    pub menu: Option<String>,
    pub default_ad: Option<String>,
    pub company_site: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAccessPointNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAccesssPointDescriptionBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAccessPointLanguageBody {
    pub language: Option<String>,
    pub is_main: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessPointCharacteristicBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAccessPointResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateAccessPointError {
    #[display(fmt = "accesspoint_already_exists")]
    AccessPointExists,
    Default(String),
}

impl ResponseError for CreateAccessPointError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateAccessPointError::AccessPointExists => {
                HttpResponse::Conflict().body("accesspoint_already_exists")
            }
            CreateAccessPointError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
