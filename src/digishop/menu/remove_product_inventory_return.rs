use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RemoveProductInventoryReturnBody {
    pub order: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveProductInventoryReturnResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum RemoveProductInventoryReturnError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for RemoveProductInventoryReturnError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RemoveProductInventoryReturnError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            RemoveProductInventoryReturnError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
