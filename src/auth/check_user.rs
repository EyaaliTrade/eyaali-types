use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckTokenBody {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckTokenResult {
    pub id: String,
    pub company_id: Option<String>,
    pub kind: String,
    pub activation_status: String,
}

#[derive(Debug, Display)]
pub enum CheckTokenError {
    #[display(fmt = "expired_token")]
    ExpiredSignature,
    #[display(fmt = "invalid_token")]
    InvalidToken,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    #[display(fmt = "user_not_verified")]
    UserNotVerified,
    #[display(fmt = "inactif_user")]
    InActifUser,

    Default(String),
}

impl ResponseError for CheckTokenError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CheckTokenError::ExpiredSignature => HttpResponse::Unauthorized().body("expired_token"),
            CheckTokenError::InvalidToken => HttpResponse::Unauthorized().body("invalid_token"),
            CheckTokenError::UserNotVerified => {
                HttpResponse::NotAcceptable().body("user_not_verified")
            }
            CheckTokenError::UserNotFound => HttpResponse::NotFound().body("user_not_found"),
            CheckTokenError::InActifUser => HttpResponse::Conflict().body("inactif_user"),
            CheckTokenError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
