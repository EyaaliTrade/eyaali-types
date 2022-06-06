use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenusBody {
    pub company: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenusResult {
    pub list: Vec<MenuAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<MenuNameAggregation>>,
    pub descriptions: Option<Vec<MenuDescriptionAggregation>>,
    pub languages: Option<Vec<MenuLanguageAggregation>>,
    pub currency: Option<MenuCurrencyAggregation>,
    pub settings: Option<MenuSettingsAggregation>,
    pub is_main: Option<bool>,
    pub is_published: Option<bool>,
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
pub struct MenuCurrencyAggregation {
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
    pub redeem_points: Option<RedeemPoints>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedeemPoints {
    pub points: i32,
    pub cash: f64,
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
