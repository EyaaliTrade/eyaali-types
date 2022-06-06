use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteStaffMemberBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteStaffMemberResult {
    pub success: bool
}

#[derive(Debug, Display)]
pub enum DeleteStaffMemberError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for DeleteStaffMemberError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DeleteStaffMemberError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DeleteStaffMemberError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}

