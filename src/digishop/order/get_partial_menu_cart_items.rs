use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetPartialMenuCartItemsBody {
    pub user: Option<String>,
    pub device: Option<String>,
    pub menu: String,
    pub discounted_cost_value: f64,
    pub delivered_cost_value: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPartialMenuCartItemsResult {
    pub list: Vec<PartialMenuCartItemsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialMenuCartItemsAggregation {
    pub id: Option<String>,
    pub product: Option<String>,
    pub options: Option<Vec<OptionValueAggregation>>,
    pub quantity: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionValueAggregation {
    pub option: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetPartialMenuCartItemsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetPartialMenuCartItemsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetPartialMenuCartItemsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetPartialMenuCartItemsError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetPartialMenuCartItemsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
