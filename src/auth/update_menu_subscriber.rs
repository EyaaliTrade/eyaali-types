use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuSubscriberBody {
    pub id: String,
    pub menu: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuSubscriberResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuSubscriberError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_subscriber_not_found")]
    MenuSubscriberNotFound,
    #[display(fmt = "email_already_exists")]
    EmailExists,
    Default(String),
}

impl ResponseError for UpdateMenuSubscriberError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuSubscriberError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuSubscriberError::MenuSubscriberNotFound => HttpResponse::Conflict().body("menu_subscriber_not_found"),
            UpdateMenuSubscriberError::EmailExists => HttpResponse::Conflict().body("email_already_exists"),
            UpdateMenuSubscriberError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
