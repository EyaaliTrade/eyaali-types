use crate::menu::get_list_menus::*;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessMenuDetailsBody {
    pub access_code: Option<String>,
    pub url_code: Option<String>,
    pub company: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessMenuDetailsResult {
    pub access_menu: Option<GetAccessMenuDetailsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessMenuDetailsAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub titles: Option<Vec<MenuTitleAggregation>>,
    pub names: Option<Vec<MenuNameAggregation>>,
    pub languages: Option<Vec<AccessMenuLanguageAggregation>>,
    pub descriptions: Option<Vec<MenuDescriptionAggregation>>,
    pub characteristics: Option<Vec<MenuCharacteristicAggregation>>,
    pub delivery_settings: Option<MenuDeliverySettingsAggregation>,
    pub logo: Option<String>,
    pub copied_logos: Option<Vec<MenuLogoAggregation>>,
    pub is_main: Option<bool>,
    pub is_published: bool,
    pub access_code: Option<String>,
    pub url_code: Option<String>,
    pub categories: Option<Vec<GetAccessMenuCategoryAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessMenuLanguageAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub is_main: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCharacteristicAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuDeliverySettingsAggregation {
    pub kind: Option<String>,
    pub limit: Option<f64>,
    pub cost: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuLogoAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessMenuCategoryAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetAccessMenuDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "access_menu_not_found")]
    AccessMenuNotFound,
    #[display(fmt = "access_menu_not_published")]
    AccessMenuNotPublished,
    Default(String),
}

impl ResponseError for GetAccessMenuDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAccessMenuDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetAccessMenuDetailsError::AccessMenuNotFound => {
                HttpResponse::NotFound().body("access_menu_not_found")
            }
            GetAccessMenuDetailsError::AccessMenuNotPublished => {
                HttpResponse::Conflict().body("access_menu_not_published")
            }
            GetAccessMenuDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
