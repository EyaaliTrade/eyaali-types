use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCustomersBody {
    pub menu: String,
    pub sorting: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCustomersResult {
    pub list: Vec<CustomerAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerAggregation {
    pub id: Option<String>,
    pub origin: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<CustomerPhoneAggregation>,
    pub address_book: Option<Vec<AddressBookAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerPhoneAggregation {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressBookAggregation {
    pub id: Option<String>,
    pub address: Option<String>,
    pub primary_phone: Option<CustomerPhoneAggregation>,
    pub secondary_phone: Option<CustomerPhoneAggregation>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Display)]
pub enum GetListCustomersError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListCustomersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCustomersError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListCustomersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
