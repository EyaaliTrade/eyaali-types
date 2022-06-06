use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductStyleBody {
    pub menu: String,
    pub product: String,
    pub border_image: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductStyleResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateProductStyleError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_theme_not_found")]
    MenuThemeNotFound,
    Default(String),
}

impl ResponseError for UpdateProductStyleError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateProductStyleError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateProductStyleError::MenuThemeNotFound => {
                HttpResponse::Conflict().body("menu_theme_not_found")
            }
            UpdateProductStyleError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
