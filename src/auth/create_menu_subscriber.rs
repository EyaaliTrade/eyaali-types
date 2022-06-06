use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateMenuSubscriberBody {
    pub menu: String,
    pub email: String,
    pub origin: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuSubscriberResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateMenuSubscriberError {
    #[display(fmt = "email_already_exists")]
    EmailExists,
    Default(String),
}

impl ResponseError for CreateMenuSubscriberError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateMenuSubscriberError::EmailExists => HttpResponse::Conflict().body("email_already_exists"),
            CreateMenuSubscriberError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
