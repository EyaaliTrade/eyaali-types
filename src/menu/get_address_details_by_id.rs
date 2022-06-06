use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAddressDetailsByIdBody {
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAddressDetailsByIdResult {
    pub address: Option<AddressAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressAggregation {
    pub id: Option<String>,
    pub road_names: Option<Vec<AddressRoadNameAggregation>>,
    pub route: Option<RouteAggregation>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub postal_code: Option<i32>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub location: Option<AddressLocationAggregation>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteAggregation {
    pub number: Option<i32>,
    pub kind: Option<String>,
    pub name: Option<String>,
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
pub enum GetAddressDetailsByIdError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetAddressDetailsByIdError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAddressDetailsByIdError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetAddressDetailsByIdError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
