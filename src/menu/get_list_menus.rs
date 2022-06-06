use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenusBody {
    pub company: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenusResult {
    pub list: Vec<MenuAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub titles: Option<Vec<MenuTitleAggregation>>,
    pub names: Option<Vec<MenuNameAggregation>>,
    pub descriptions: Option<Vec<MenuDescriptionAggregation>>,
    pub sites: Option<Vec<MenuSiteAggregation>>,
    pub languages: Option<Vec<MenuLanguageAggregation>>,
    pub currency: Option<MenuCurrencyAggregation>,
    pub delivery_settings: Option<MenuDeliverySettingsAggregation>,
    pub access_code: Option<String>,
    pub url_code: Option<String>,
    pub qr_code_picture: Option<QrCodePictureAggregation>,
    pub logo: Option<MenuLogoAggregation>,
    pub screenshots: Option<Vec<MenuScreenShotAggregation>>,
    pub copied_logos: Option<Vec<MenuLogoAggregation>>,
    pub characteristics: Option<Vec<MenuCharacteristicAggregation>>,
    pub default_prep_time: Option<DefaultPrepTimeAggregation>,
    pub is_main: Option<bool>,
    pub is_published: Option<bool>,
    pub categories_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuTitleAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuSiteAggregation {
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuLanguageAggregation {
    pub language: Option<LanguageAggregation>,
    pub is_main: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguageAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCurrencyAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuDeliverySettingsAggregation {
    pub kind: Option<String>,
    pub limit: Option<f64>,
    pub cost: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QrCodePictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuLogoAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuScreenShotAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
    pub is_visible: Option<bool>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCharacteristicAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultPrepTimeAggregation {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Debug, Display)]
pub enum GetListMenusError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListMenusError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMenusError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListMenusError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
