use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreatePromoCodeBody {
    pub menu: String,
    pub code: String,
    pub kind: String,
    pub values: Option<PromoCodeValuesBody>,
    pub limits: Option<PromoCodeLimitsBody>,
    pub requirements: Option<PromoCodeRequirementsBody>,
    pub targets: Option<PromoCodeTargetsBody>,
    pub filters: Option<PromoCodeFiltersBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeValuesBody {
    pub percentage: Option<i32>,
    pub price: Option<PriceBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceBody {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeLimitsBody {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub maximum_uses: Option<i32>,
    pub is_used_once: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeRequirementsBody {
    pub kind: Option<String>,
    pub minimum_cart_total_price: Option<f64>,
    pub minimum_cart_total_quantity: Option<f64>,
    pub minimum_past_orders_total_price: Option<f64>,
    pub minimum_past_orders_total_quantity: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeTargetsBody {
    pub kind: Option<String>,
    pub customers: Option<Vec<PromoCodeCustomerIdBody>>,
    pub groups: Option<Vec<PromoCodeGroupIdBody>>,
    pub subscribers: Option<Vec<PromoCodeSubscriberIdBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeCustomerIdBody {
    pub customer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeGroupIdBody {
    pub group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeSubscriberIdBody {
    pub subscriber: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeFiltersBody {
    pub kind: Option<String>,
    pub collections: Option<Vec<PromoCodeCollectionIdBody>>,
    pub categories: Option<Vec<PromoCodeCategoryIdBody>>,
    pub products: Option<Vec<PromoCodeProductIdBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeCollectionIdBody {
    pub collection: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeCategoryIdBody {
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeProductIdBody {
    pub product: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePromoCodeResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreatePromoCodeError {
    #[display(fmt = "code_already_exists")]
    CodeExists,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for CreatePromoCodeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreatePromoCodeError::CodeExists => HttpResponse::Conflict().body("code_already_exists"),
            CreatePromoCodeError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            CreatePromoCodeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
