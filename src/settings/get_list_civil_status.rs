use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCivilStatusBody {
    pub language_code: String,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCivilStatusResult {
    pub list: Vec<CivilStatusAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CivilStatusAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListCivilStatusError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListCivilStatusError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCivilStatusError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListCivilStatusError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
