use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetEasyPayListRegionsResult {
    pub list: Vec<EasyPayRegionAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EasyPayRegionAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetEasyPayListRegionsError {
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetEasyPayListRegionsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetEasyPayListRegionsError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetEasyPayListRegionsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
