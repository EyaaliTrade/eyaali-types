use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCompanySiteByIdBody {
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCompanySiteByIdResult {
    pub company_site: Option<CompanySiteAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanySiteAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<CompanySiteNameAggregation>>,
    pub descriptions: Option<Vec<CompanySiteDescriptionAggregation>>,
    pub address: Option<String>,
    pub phone: Option<CompanySitePhoneAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanySiteNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanySiteDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanySitePhoneAggregation {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetCompanySiteByIdError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetCompanySiteByIdError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCompanySiteByIdError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCompanySiteByIdError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
