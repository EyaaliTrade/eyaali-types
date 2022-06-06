use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDTOneListOperatorsBody {
    pub country_iso_code: String,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDTOneListOperatorsResult {
    pub list: Vec<DTOneOperatorAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DTOneOperatorAggregation {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetDTOneListOperatorsError {
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetDTOneListOperatorsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetDTOneListOperatorsError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetDTOneListOperatorsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
