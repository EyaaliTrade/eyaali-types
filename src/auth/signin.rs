use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSignInBody {
    pub login: String,
    pub password: String,
    pub kind: Option<String>,
    pub company_kind: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSignInResult {
    pub id: String,
    pub token: String,
    pub kind: String,
    pub activation_status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAggregation {
    pub id: Option<String>,
    pub kind: String,
    pub password: String,
    pub company: Option<String>,
    pub account_activation: Option<AccountActivationAggregation>,
    pub account_verification: Option<AccountVerificationAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountActivationAggregation {
    pub activation_date: Option<String>,
    pub activation_status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountVerificationAggregation {
    pub is_verified: Option<bool>,
    pub verification_code: Option<String>,
}

#[derive(Debug, Display)]
pub enum UserSignInError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "error_hashing_password")]
    HashPassword,
    #[display(fmt = "invalid_password")]
    InvalidPassword,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    #[display(fmt = "user_not_verified")]
    UserNotVerified,
    #[display(fmt = "inactif_user")]
    InActifUser,
    Default(String),
}

impl ResponseError for UserSignInError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserSignInError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UserSignInError::HashPassword => {
                HttpResponse::InternalServerError().body("error_hashing_password")
            }
            UserSignInError::InvalidPassword => {
                HttpResponse::Unauthorized().body("invalid_password")
            }
            UserSignInError::UserNotFound => HttpResponse::Conflict().body("user_not_found"),
            UserSignInError::UserNotVerified => HttpResponse::Gone().body("user_not_verified"),
            UserSignInError::InActifUser => HttpResponse::Forbidden().body("inactif_user"),
            UserSignInError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
