use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocialAuthenticationBody {
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocialAuthenticationResult {
    pub id: String,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub email: Option<String>,
    pub birthday: Option<String>,
    pub gender: Option<String>,
}

#[derive(Debug, Display)]
pub enum SocialAuthenticationError {
    #[display(fmt = "invalid_token")]
    InvalidToken,
    Default(String),
}

impl ResponseError for SocialAuthenticationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SocialAuthenticationError::InvalidToken => {
                HttpResponse::NotAcceptable().body("invalid_token")
            }
            SocialAuthenticationError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
