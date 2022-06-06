use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListActivationStatusBody {
    pub language_code: String,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListActivationStatusResult {
    pub list: Vec<ActivationStatusAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivationStatusAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListActivationStatusError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListActivationStatusError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListActivationStatusError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListActivationStatusError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
