use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCustomerAddressDetailsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCustomerAddressDetailsResult {
    pub customer_address: Option<CustomerAddressDetailsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerAddressDetailsAggregation {
    pub id: Option<String>,
    pub address: Option<String>,
    pub primary_phone: Option<CustomerPhoneAggregation>,
    pub secondary_phone: Option<CustomerPhoneAggregation>,
    pub is_default: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerPhoneAggregation {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetCustomerAddressDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "customer_address_not_found")]
    CustomerAddressNotFound,
    Default(String),
}

impl ResponseError for GetCustomerAddressDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCustomerAddressDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCustomerAddressDetailsError::CustomerAddressNotFound => {
                HttpResponse::Conflict().body("customer_address_not_found")
            }
            GetCustomerAddressDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
