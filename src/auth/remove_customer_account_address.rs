use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RemoveCustomerAccountAddressBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveCustomerAccountAddressResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum RemoveCustomerAccountAddressError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "customer_address_not_found")]
    CustomerAddressNotFound,
    Default(String),
}

impl ResponseError for RemoveCustomerAccountAddressError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RemoveCustomerAccountAddressError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            RemoveCustomerAccountAddressError::CustomerAddressNotFound => HttpResponse::Conflict().body("customer_address_not_found"),
            RemoveCustomerAccountAddressError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
