use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDescriptionDetailsByIdBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDescriptionDetailsByIdResult {
    pub description: Option<DescriptionAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetDescriptionDetailsByIdError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetDescriptionDetailsByIdError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetDescriptionDetailsByIdError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetDescriptionDetailsByIdError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
