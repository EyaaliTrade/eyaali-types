use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListLegalFormsBody {
    pub language_code: String,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListLegalFormsResult {
    pub list: Vec<LegalFormAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegalFormAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListLegalFormsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListLegalFormsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListLegalFormsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListLegalFormsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
