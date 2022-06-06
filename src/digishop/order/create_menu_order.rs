use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateMenuOrderBody {
    pub id: String,
    pub user: String,
    pub menu: String,
    pub delivery_method: String,
    pub delivery_address: Option<String>,
    pub payment_method: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuOrderResult {
    pub reference: String,
}

#[derive(Debug, Display)]
pub enum CreateMenuOrderError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "reference_already_exists")]
    ReferenceExists,
    Default(String),
}

impl ResponseError for CreateMenuOrderError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateMenuOrderError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateMenuOrderError::ReferenceExists => {
                HttpResponse::Conflict().body("reference_already_exists")
            }
            CreateMenuOrderError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
