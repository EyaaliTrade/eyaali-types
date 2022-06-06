
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuOrderPrepTimeBody {
    pub order: String,
    pub order_prep_time: Option<i32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuOrderPrepTimeResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuOrderPrepTimeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_order_not_found")]
    MenuOrderNotFound,
    Default(String),
}

impl ResponseError for UpdateMenuOrderPrepTimeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuOrderPrepTimeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuOrderPrepTimeError::MenuOrderNotFound => {
                HttpResponse::NotFound().body("action_not_allowed")
            }
            UpdateMenuOrderPrepTimeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}