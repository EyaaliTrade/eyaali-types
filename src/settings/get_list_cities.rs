use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCitiesBody {
    pub country_id: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCitiesResult {
    pub list: Vec<CityAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CityAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListCitiesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListCitiesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCitiesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListCitiesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
