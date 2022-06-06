use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuOrdersBody {
    pub menu: String,
    pub vendor_id: Option<String>,
    pub vendor_products_id: Option<Vec<String>>,
    pub reference: Option<String>,
    pub filter: Option<String>,
    pub order_status: Option<String>,
    pub payment_status: Option<String>,
    pub date: Option<DateFilterBody>,
    pub sorting: Option<String>,
    pub page_number: i32,
    pub page_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateFilterBody {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuOrdersResult {
    pub list: Vec<MenuOrderAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuOrderAggregation {
    pub id: Option<String>,
    pub user: Option<String>,
    pub delivery_method: Option<String>,
    pub delivery_address: Option<String>,
    pub reference: Option<String>,
    pub discounted_cost: Option<f64>,
    pub delivered_cost: Option<f64>,
    pub order_status: Option<String>,
    pub payment_status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub confirmation_payment: Option<String>
}

#[derive(Debug, Display)]
pub enum GetListMenuOrdersError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetListMenuOrdersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMenuOrdersError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListMenuOrdersError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetListMenuOrdersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
