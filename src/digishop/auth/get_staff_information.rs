use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetStaffInformationBody {
    pub id: String,
    pub language_code: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetStaffInformationResult {
    pub information: Option<StaffMemberAggregation>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaffMemberAggregation {
    pub id: Option<String>,
    pub email: Option<String>,
    pub role: Option<RoleAggregation>,
    pub permissions: Option<Vec<StaffPermissionAggregation>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaffPermissionAggregation{
    pub kind: Option<String>,
    pub values: Option<Vec<PermissionAggregation>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleAggregation {
    pub id: Option<String>,
    pub kind: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionAggregation {
    pub id: Option<String>,
    pub kind: Option<String>,
    pub names: Option<Vec<PermissionNameAggregation>>,
    pub descriptions: Option<Vec<PermissionDescriptionAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetStaffInformationError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "staff_member_not_found")]
    StaffMemberNotFound,
    Default(String),
}

impl ResponseError for GetStaffInformationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetStaffInformationError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetStaffInformationError::StaffMemberNotFound => {
                HttpResponse::NotFound().body("staff_member_not_found")
            }
            GetStaffInformationError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}