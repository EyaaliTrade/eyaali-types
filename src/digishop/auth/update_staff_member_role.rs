use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateStaffMemberRoleBody {
    pub member: String,
    pub role_id: String

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateStaffMemberRoleResult {
    pub success: bool
}

#[derive(Debug, Display)]
pub enum UpdateStaffMemberRoleError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "staff_member_not_found")]
    StaffMemberNotFound,
    Default(String),
}

impl ResponseError for UpdateStaffMemberRoleError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateStaffMemberRoleError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateStaffMemberRoleError::StaffMemberNotFound => {
                HttpResponse::NotFound().body("staff_member_not_found")
            }
            UpdateStaffMemberRoleError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}