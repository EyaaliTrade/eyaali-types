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
    pub titles: Option<Vec<UpdateMenuTitleBody>>,
    pub names: Option<Vec<UpdateMenuNameBody>>,
    pub descriptions: Option<Vec<UpdateMenuDescriptionBody>>,
    pub sites: Option<Vec<MenuSiteBody>>,
    pub languages: Option<Vec<MenuLanguageBody>>,
    pub currency: Option<String>,
    pub delivery_settings: Option<MenuDeliverySettingsBody>,
    pub logo: Option<String>,
    pub copied_logos: Option<Vec<MenuCopiedLogoBody>>,
    pub screenshots: Option<Vec<MenuScreenshotBody>>,
    pub characteristics: Option<Vec<MenuCharacteristicBody>>,
    pub default_prep_time: Option<DefaultPrepTimeBody>,
    pub is_published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuTitleBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
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
pub struct MenuSiteBody {
    pub site: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuLanguageBody {
    pub language: Option<String>,
    pub is_main: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuDeliverySettingsBody {
    pub kind: Option<String>,
    pub limit: Option<f64>,
    pub cost: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCopiedLogoBody {
    pub id: Option<String>,
    pub quality: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuScreenshotBody {
    pub file_id: Option<String>,
    pub is_visible: Option<bool>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCharacteristicBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DefaultPrepTimeBody {
    pub min: Option<i32>,
    pub max: Option<i32>
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
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    #[display(fmt = "identifier_is_not_alphabectic")]
    IdentifierIsNotAplhabetic,
    #[display(fmt = "maximum_published_menu_reached")]
    MaximumReached,
    Default(String),
}

impl ResponseError for UpdateMenuError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuError::MenuNotFound => HttpResponse::Conflict().body("menu_not_found"),
            UpdateMenuError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            UpdateMenuError::IdentifierIsNotAplhabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabectic")
            }
            UpdateMenuError::MaximumReached => {
                HttpResponse::Gone().body("maximum_published_menu_reached")
            }
            UpdateMenuError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
