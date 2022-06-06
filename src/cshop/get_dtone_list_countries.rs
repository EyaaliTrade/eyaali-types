use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDTOneListCountriesBody {
    pub page: i32,
    pub per_page: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDTOneListCountriesResult {
    pub list: Vec<DTOneCountryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DTOneCountryAggregation {
    pub iso_code: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetDTOneListCountriesError {
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetDTOneListCountriesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetDTOneListCountriesError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetDTOneListCountriesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
