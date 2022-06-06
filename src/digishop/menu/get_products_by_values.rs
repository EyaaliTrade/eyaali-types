use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductsByValuesBody {
    pub menu: String,
    pub category: Option<String>,
    pub language_code: Option<String>,
    pub options: Option<Vec<OptionKindValuesBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionKindValuesBody {
    pub kind: Option<String>,
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductsByValuesResult {
    pub products: Option<Vec<String>>,
}

#[derive(Debug, Display)]
pub enum GetProductsByValuesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductsByValuesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductsByValuesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductsByValuesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
