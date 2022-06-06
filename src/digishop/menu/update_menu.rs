use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuBody {
    pub id: String,
    pub number_of_authorized_menus: i32,
    pub number_of_published_menus: i32,
    pub company: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<UpdateMenuNameBody>>,
    pub descriptions: Option<Vec<UpdateMenuDescriptionBody>>,
    pub languages: Option<Vec<MenuLanguageBody>>,
    pub currency: Option<String>,
    pub settings: Option<MenuSettingsBody>,
    pub is_published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuLanguageBody {
    pub language: Option<String>,
    pub is_main: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuSettingsBody {
    pub currency_display: Option<String>,
    pub price_precision: Option<i32>,
    pub stock_is_active: Option<bool>,
    pub pod_is_enabled: Option<bool>,
    pub pis_is_enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuResult {
    pub success: bool,
    pub is_reached: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_not_found")]
    MenuNotFound,
    #[display(fmt = "maximum_published_menu_reached")]
    MaximumReached,
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    #[display(fmt = "identifier_is_not_alphabetic")]
    IdentifierIsNotAlphabetic,
    Default(String),
}

impl ResponseError for UpdateMenuError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuError::MenuNotFound => HttpResponse::Conflict().body("menu_not_found"),
            UpdateMenuError::MaximumReached => {
                HttpResponse::Gone().body("maximum_published_menu_reached")
            }
            UpdateMenuError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            UpdateMenuError::IdentifierIsNotAlphabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabetic")
            }
            UpdateMenuError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
