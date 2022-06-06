use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateMenuBody {
    pub number_of_authorized_menus: i32,
    pub number_of_published_menus: i32,
    pub company: Option<String>,
    pub identifier: Option<String>,
    pub titles: Option<Vec<CreateMenuTitleBody>>,
    pub names: Option<Vec<CreateMenuNameBody>>,
    pub descriptions: Option<Vec<CreateMenuDescriptionBody>>,
    pub sites: Option<Vec<MenuSiteBody>>,
    pub languages: Option<Vec<MenuLanguageBody>>,
    pub currency: Option<String>,
    pub delivery_settings: Option<MenuDeliverySettingsBody>,
    pub logo: Option<String>,
    pub copied_logos: Option<Vec<MenuCopiedLogoBody>>,
    pub characteristics: Option<Vec<MenuCharacteristicBody>>,
    pub default_prep_time: Option<DefaultPrepTimeBody>,
    pub is_published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuTitleBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuDescriptionBody {
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
pub struct CreateMenuResult {
    pub id: String,
    pub is_reached: bool,
}

#[derive(Debug, Display)]
pub enum CreateMenuError {
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    #[display(fmt = "maximum_published_menu_reached")]
    MaximumReached,
    #[display(fmt = "identifier_is_not_alphabectic")]
    IdentifierIsNotAplhabetic,
    Default(String),
}

impl ResponseError for CreateMenuError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateMenuError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            CreateMenuError::MaximumReached => {
                HttpResponse::Gone().body("maximum_published_menu_reached")
            }
            CreateMenuError::IdentifierIsNotAplhabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabectic")
            }
            CreateMenuError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
