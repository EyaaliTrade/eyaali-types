use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteRoleBody {
 pub id: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteRoleResult {
    pub success: bool
}

#[derive(Debug, Display)]
pub enum DeleteRoleError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteRoleError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteRoleError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteRoleError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}