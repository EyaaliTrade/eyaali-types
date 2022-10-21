use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuOrderByIdBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuOrderByIdResult {
    pub id: Option<String>,
    pub menu: Option<String>,
    pub user: Option<String>,
    pub delivery_method: Option<String>,
    pub delivery_address: Option<String>,
    pub reference: Option<String>,
    pub confirmation_payment: Option<String>,
    pub discounted_cost: Option<f64>,
    pub delivered_cost: Option<f64>,
    pub order_status: Option<String>,
    pub payment_status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub comment: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuOrderAggregation {
    pub id: Option<String>,
    pub menu: Option<String>,
    pub user: Option<String>,
    pub delivery_method: Option<String>,
    pub delivery_address: Option<String>,
    pub reference: Option<String>,
    pub confirmation_payment: Option<String>,
    pub discounted_cost: Option<f64>,
    pub delivered_cost: Option<f64>,
    pub order_status: Option<String>,
    pub payment_status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetMenuOrderByIdError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "order_not_found")]
    OrderNotFound,
    Default(String),
}

impl ResponseError for GetMenuOrderByIdError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuOrderByIdError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuOrderByIdError::OrderNotFound => {
                HttpResponse::Conflict().body("order_not_found")
            }
            GetMenuOrderByIdError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
