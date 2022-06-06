use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct ClearMenuCartBody {
    pub user: Option<String>,
    pub device: Option<String>,
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearMenuCartResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum ClearMenuCartError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    #[display(fmt = "cart_already_empty")]
    CartAlreadyEmpty,
    Default(String),
}

impl ResponseError for ClearMenuCartError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ClearMenuCartError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            ClearMenuCartError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            ClearMenuCartError::CartAlreadyEmpty => {
                HttpResponse::Conflict().body("cart_already_empty")
            }
            ClearMenuCartError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
