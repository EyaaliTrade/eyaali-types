use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportCustomersBody {
    pub menu: String,
    pub company: String
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CustomerRecord {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub postal_code: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportCustomersResult {
    pub file_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerAggregation {
    pub id: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<CustomerPhoneAggregation>,
    pub address: Option<AddressBookAggregation>,
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
pub enum ExportCustomersError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for ExportCustomersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ExportCustomersError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            ExportCustomersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
