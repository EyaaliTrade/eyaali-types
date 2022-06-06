use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPartialCustomerDetailsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPartialCustomerDetailsResult {
    pub customer: Option<PartialCustomerDetailsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialCustomerDetailsAggregation {
    pub id: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub picture: Option<CustomerPictureUrlAggregation>,
    pub phone: Option<CustomerPhoneAggregation>,
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

#[derive(Debug, Display)]
pub enum GetPartialCustomerDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    Default(String),
}

impl ResponseError for GetPartialCustomerDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetPartialCustomerDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetPartialCustomerDetailsError::UserNotFound => {
                HttpResponse::Conflict().body("user_not_found")
            }
            GetPartialCustomerDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
