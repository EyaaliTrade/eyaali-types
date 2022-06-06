use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductVariantNamesBody {
    pub product: Option<String>,
    pub variant: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductVariantNamesResult {
    pub names: Option<Vec<NameAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetProductVariantNamesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductVariantNamesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductVariantNamesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductVariantNamesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
