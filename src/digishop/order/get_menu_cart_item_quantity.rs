use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetMenuCartItemQuantityBody {
    pub user: Option<String>,
    pub device: Option<String>,
    pub menu: String,
    pub site: Option<String>,
    pub product: String,
    pub options: Option<Vec<OptionValueBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionValueBody {
    pub option: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuCartItemQuantityResult {
    pub quantity: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuCartItemQuantityAggregation {
    pub quantity: f64,
}

#[derive(Debug, Display)]
pub enum GetMenuCartItemQuantityError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetMenuCartItemQuantityError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuCartItemQuantityError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuCartItemQuantityError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetMenuCartItemQuantityError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
