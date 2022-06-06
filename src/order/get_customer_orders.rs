use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetCustomerOrdersBody {
    pub customer: String,
    pub menu: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetCustomerOrdersResult {
    pub list: Option<CustomerOrders>
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CustomerOrders {
    pub total_amnount: f64,
    pub total_purchase: i32,
    pub total_ordered_items: i32,
    //pub orders: Vec<CustomerOrderDetails>
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CustomerOrderDetails {
    pub id: Option<String>,
    pub reference: Option<String>,
    pub order_date: Option<String>
}

#[derive(Debug, Display)]
pub enum GetCustomerOrdersError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetCustomerOrdersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCustomerOrdersError::InvalidObjectId => HttpResponse::NotAcceptable().body("not_acceptable"),
            GetCustomerOrdersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
