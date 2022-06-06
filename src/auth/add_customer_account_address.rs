use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AddCustomerAccountAddressBody {
    pub menu: String,
    pub customer: String,
    pub address: Option<CreateCustomerAddressBody>,
    pub primary_phone: Option<CustomerPhoneBody>,
    pub secondary_phone: Option<CustomerPhoneBody>,
    pub is_default: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCustomerAddressBody {
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
pub struct CreateAddressRoadNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
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
pub struct AddCustomerAccountAddressResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum AddCustomerAccountAddressError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    Default(String),
}

impl ResponseError for AddCustomerAccountAddressError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AddCustomerAccountAddressError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            AddCustomerAccountAddressError::UserNotFound => HttpResponse::Conflict().body("user_not_found"),
            AddCustomerAccountAddressError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
