use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetMenuCartDetailsBody {
    pub user: Option<String>,
    pub device: Option<String>,
    pub menu: String,
    pub discounted_cost_value: f64,
    pub delivered_cost_value: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuCartDetailsResult {
    pub count: Option<i32>,
    pub items: Option<Vec<CartItemAggregation>>,
    pub original_total_price: Option<PriceAggregation>,
    pub total_discounted_cost: Option<PriceAggregation>,
    pub total_delivered_cost: Option<PriceAggregation>,
    pub total_price: Option<PriceAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCartAggregation {
    pub count: Option<i32>,
    pub items: Option<Vec<CartItemAggregation>>,
    pub original_total_price: Option<PriceAggregation>,
    pub total_discounted_cost: Option<PriceAggregation>,
    pub total_delivered_cost: Option<PriceAggregation>,
    pub total_price: Option<PriceAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartItemAggregation {
    pub id: Option<String>,
    pub product: Option<String>,
    pub options: Option<Vec<CartProductOptionAggregation>>,
    pub quantity: Option<f64>,
    pub unit_price: Option<PriceAggregation>,
    pub total_price: Option<PriceAggregation>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceAggregation {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartProductOptionAggregation {
    pub option: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetMenuCartDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetMenuCartDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuCartDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuCartDetailsError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetMenuCartDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
