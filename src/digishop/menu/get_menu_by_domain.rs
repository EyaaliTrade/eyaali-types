use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuByDomainBody {
    pub company: String,
    pub identifier: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuByDomainResult {
    pub menu: MenuByDomainAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuByDomainAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<MenuNameAggregation>>,
    pub descriptions: Option<Vec<MenuDescriptionAggregation>>,
    pub languages: Option<Vec<MenuLanguageAggregation>>,
    pub settings: Option<MenuSettingsAggregation>,
    pub legality_settings: Option<MenuLegalitySettingsAggregation>,
    pub price_range: Option<PriceRangeAggregation>,
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
pub struct MenuSettingsAggregation {
    pub currency_display: Option<String>,
    pub price_precision: Option<i32>,
    pub stock_is_active: Option<bool>,
    pub pod_is_enabled: Option<bool>,
    pub pis_is_enabled: Option<bool>,
    pub pdt_has_multi_categories: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedeemPoints {
    pub points: i32,
    pub cash: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuLegalitySettingsAggregation {
    pub consent_is_required: Option<bool>,
    pub approval_is_required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceRangeAggregation {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Display)]
pub enum GetMenuByDomainError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_not_found")]
    MenuNotFound,
    Default(String),
}

impl ResponseError for GetMenuByDomainError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuByDomainError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuByDomainError::MenuNotFound => {
                HttpResponse::Conflict().body("menu_not_found")
            }
            GetMenuByDomainError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
