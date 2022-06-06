use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDTOneListServicesResult {
    pub list: Vec<DTOneServiceAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DTOneServiceAggregation {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetDTOneListServicesError {
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetDTOneListServicesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetDTOneListServicesError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetDTOneListServicesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
