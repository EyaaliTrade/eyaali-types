use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DeleteCompanySettingsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteCompanySettingsResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum DeleteCompanySettingsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteCompanySettingsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteCompanySettingsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteCompanySettingsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
