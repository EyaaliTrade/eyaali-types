use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAddressDetailsByLanguageBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAddressDetailsByLanguageResult {
    pub address: Option<AddressAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressAggregation {
    pub id: Option<String>,
    pub road_name: Option<AddressRoadNameAggregation>,
    pub road_names: Option<Vec<AddressRoadNameAggregation>>,
    pub postal_code: Option<i32>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub location: Option<AddressLocationAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressRoadNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressLocationAggregation {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Display)]
pub enum GetAddressDetailsByLanguageError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetAddressDetailsByLanguageError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAddressDetailsByLanguageError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetAddressDetailsByLanguageError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
