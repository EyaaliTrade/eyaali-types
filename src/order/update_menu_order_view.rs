use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuOrderViewBody {
    pub order: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuOrderViewResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuOrderViewError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "order_not_found")]
    OrderNotFound,
    Default(String),
}

impl ResponseError for UpdateMenuOrderViewError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuOrderViewError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuOrderViewError::OrderNotFound => {
                HttpResponse::Conflict().body("order_not_found")
            }
            UpdateMenuOrderViewError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
