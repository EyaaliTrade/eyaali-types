use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCountriesBody {
    pub name: Option<String>,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCountriesResult {
    pub list: Vec<CountryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountryAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListCountriesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListCountriesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCountriesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListCountriesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
