use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct EditCustomerAccountAddressBody {
    pub id: String,
    pub address: Option<UpdateCustomerAddressBody>,
    pub primary_phone: Option<CustomerPhoneBody>,
    pub secondary_phone: Option<CustomerPhoneBody>,
    pub is_default: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCustomerAddressBody {
    pub id: Option<String>,
    pub floor: Option<String>,
    pub building: Option<String>,
    pub route: Option<RouteBody>,    
    pub postal_code: Option<i32>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub location: Option<AddressLocationBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteBody {
    pub number: Option<i32>,
    pub kind: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressLocationBody {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerPhoneBody {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditCustomerAccountAddressResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum EditCustomerAccountAddressError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "customer_address_not_found")]
    CustomerAddressNotFound,
    Default(String),
}

impl ResponseError for EditCustomerAccountAddressError {
    fn error_response(&self) -> HttpResponse {
        match self {
            EditCustomerAccountAddressError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            EditCustomerAccountAddressError::CustomerAddressNotFound => HttpResponse::Conflict().body("customer_address_not_found"),
            EditCustomerAccountAddressError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
