use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetWorkloadsStatusBody {
    pub domain: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetWorkloadsStatusResult {
    pub init: Option<String>,
    pub back: Option<BackWorkloadsStatusAggregation>,
    pub front: Option<FrontWorkloadsStatusAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackWorkloadsStatusAggregation {
    pub auth: Option<String>,
    pub gateway: Option<String>,
    pub menu: Option<String>,
    pub order: Option<String>,
    pub payment: Option<String>,
    pub settings: Option<String>,
    pub stats: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrontWorkloadsStatusAggregation {
    pub admin: Option<String>,
    pub manager: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetWorkloadsStatusError {
    #[display(fmt = "domain_not_found")]
    DomainNotFound,
    Default(String),
}

impl ResponseError for GetWorkloadsStatusError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetWorkloadsStatusError::DomainNotFound => HttpResponse::Conflict().body("domain_not_found"),
            GetWorkloadsStatusError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
