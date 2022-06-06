use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RemoveCustomerAddressBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveCustomerAddressResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum RemoveCustomerAddressError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "action_not_allowed")]
    ActionNotAllowed,
    #[display(fmt = "customer_address_not_found")]
    CustomerAddressNotFound,
    Default(String),
}

impl ResponseError for RemoveCustomerAddressError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RemoveCustomerAddressError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            RemoveCustomerAddressError::ActionNotAllowed => {
                HttpResponse::Forbidden().body("action_not_allowed")
            }
            RemoveCustomerAddressError::CustomerAddressNotFound => HttpResponse::Conflict().body("customer_address_not_found"),
            RemoveCustomerAddressError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
