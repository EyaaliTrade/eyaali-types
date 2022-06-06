use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveAssignmentBody {
    pub role_id: String,
    pub staff_member: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveAssignmentResult {
    pub success: bool
}


#[derive(Debug, Display)]
pub enum RemoveAssignmentfError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for RemoveAssignmentfError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RemoveAssignmentfError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            RemoveAssignmentfError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}