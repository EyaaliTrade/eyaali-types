use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetListPastMenuOrdersBody {
    pub menus: Vec<String>,
    pub past_days_count: Option<i32>,
    pub day: Option<String>,
    pub reference: Option<String>,
    pub menu: Option<String>,
    pub page_number: Option<i32>,
    pub page_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListPastMenuOrdersResult {
    pub list: Vec<PastMenuOrderAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PastMenuOrderAggregation {
    pub day: Option<String>,
    pub orders: Vec<MenuOrderAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuOrderAggregation {
    pub id: Option<String>,
    pub kind: Option<String>,
    pub reference: Option<String>,
    pub order_date: Option<String>,
    pub customer_informations: Option<CustomerInformationsAggregation>,
    pub customer_notes: Option<String>,
    pub order_status: Option<String>,
    pub payment_status: Option<String>,
    pub updated_date: Option<String>,
    pub is_viewed: Option<bool>,
    pub details: Option<MenuOrderDetailsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerInformationsAggregation {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub phone: Option<CustomerPhoneAggregation>,
    pub position: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerPhoneAggregation {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuOrderDetailsAggregation {
    pub order: Option<String>,
    pub menu: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListPastMenuOrdersError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListPastMenuOrdersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListPastMenuOrdersError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListPastMenuOrdersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
