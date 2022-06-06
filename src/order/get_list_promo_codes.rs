use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetListPromoCodesBody {
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListPromoCodesResult {
    pub list: Vec<PromoCodeAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
    pub kind: Option<String>,
    pub values: Option<PromoCodeValuesAggregation>,
    pub limits: Option<PromoCodeLimitsAggregation>,
    pub requirements: Option<PromoCodeRequirementsAggregation>,
    pub targets: Option<PromoCodeTargetsAggregation>,
    pub filters: Option<PromoCodeFiltersAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeValuesAggregation {
    pub percentage: Option<i32>,
    pub price: Option<PromoCodePriceAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodePriceAggregation {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeLimitsAggregation {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub maximum_uses: Option<i32>,
    pub is_used_once: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeRequirementsAggregation {
    pub kind: Option<String>,
    pub minimum_cart_total_price: Option<f64>,
    pub minimum_cart_total_quantity: Option<f64>,
    pub minimum_past_orders_total_price: Option<f64>,
    pub minimum_past_orders_total_quantity: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeTargetsAggregation {
    pub kind: Option<String>,
    pub customers: Option<Vec<PromoCodeCustomerIdAggregation>>,
    pub groups: Option<Vec<PromoCodeGroupIdAggregation>>,
    pub subscribers: Option<Vec<PromoCodeSubscriberIdAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeCustomerIdAggregation {
    pub customer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeGroupIdAggregation {
    pub group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeSubscriberIdAggregation {
    pub subscriber: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeFiltersAggregation {
    pub kind: Option<String>,
    pub collections: Option<Vec<PromoCodeCollectionIdAggregation>>,
    pub categories: Option<Vec<PromoCodeCategoryIdAggregation>>,
    pub products: Option<Vec<PromoCodeProductIdAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeCollectionIdAggregation {
    pub collection: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeCategoryIdAggregation {
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoCodeProductIdAggregation {
    pub product: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListPromoCodesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListPromoCodesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListPromoCodesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListPromoCodesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
