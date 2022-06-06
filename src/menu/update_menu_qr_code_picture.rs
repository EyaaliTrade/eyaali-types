use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuQrCodePictureBody {
    pub menu: String,
    pub domain: String,
    pub identifier: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuQrCodePictureResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuQrCodePictureError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_not_found")]
    MenuNotFound,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for UpdateMenuQrCodePictureError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuQrCodePictureError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuQrCodePictureError::MenuNotFound => {
                HttpResponse::Conflict().body("menu_not_found")
            }
            UpdateMenuQrCodePictureError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            UpdateMenuQrCodePictureError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
