use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetListCurrentMenuOrdersBody {
    pub menus: Vec<String>,
    pub filter: String,
    pub reference: Option<String>,
    pub menu: Option<String>,
    pub page_number: Option<i32>,
    pub page_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCurrentMenuOrdersResult {
    pub list: Vec<MenuOrderAggregation>,
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
pub enum GetListCurrentMenuOrdersError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListCurrentMenuOrdersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCurrentMenuOrdersError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListCurrentMenuOrdersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
