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
    pub names: Option<Vec<CreateMenuNameBody>>,
    pub descriptions: Option<Vec<CreateMenuDescriptionBody>>,
    pub languages: Option<Vec<MenuLanguageBody>>,
    pub currency: Option<String>,
    pub settings: Option<MenuSettingsBody>,
    pub is_published: Option<bool>,
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
pub struct RedeemPoints {
    pub points: i32,
    pub cash: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductPriceBody {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuResult {
    pub id: String,
    pub is_reached: bool,
}

#[derive(Debug, Display)]
pub enum CreateMenuError {
    #[display(fmt = "maximum_published_menu_reached")]
    MaximumReached,
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    #[display(fmt = "identifier_is_not_alphabetic")]
    IdentifierIsNotAlphabetic,
    Default(String),
}

impl ResponseError for CreateMenuError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateMenuError::MaximumReached => {
                HttpResponse::Gone().body("maximum_published_menu_reached")
            }
            CreateMenuError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            CreateMenuError::IdentifierIsNotAlphabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabetic")
            }
            CreateMenuError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
