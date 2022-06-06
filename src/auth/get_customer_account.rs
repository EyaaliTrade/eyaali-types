use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCustomerAccountBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCustomerAccountResult {
    pub account: CustomerAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerAggregation {
    pub id: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<String>,
    pub gender: Option<String>,
    pub picture: Option<CustomerPictureUrlAggregation>,
    pub phone: Option<CustomerPhoneAggregation>,
    pub is_notified: Option<bool>,
    pub address_book: Option<Vec<AddressBookAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerPictureUrlAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
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
pub enum GetCustomerAccountError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    Default(String),
}

impl ResponseError for GetCustomerAccountError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCustomerAccountError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCustomerAccountError::UserNotFound => HttpResponse::Conflict().body("user_not_found"),
            GetCustomerAccountError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
