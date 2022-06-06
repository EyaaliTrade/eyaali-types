use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListRolesBody {
    pub menu:  String,
    pub id:  Option<String>,
    pub language_code:  Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListRolesResult {
    pub list: Vec<RoleAggregation>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleAggregation {
    pub id: Option<String>,
    pub kind: Option<String>,
    pub permissions: Vec<PermissionAggregation>,
    pub staff: Vec<StaffMemberAggregation>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaffMemberAggregation {
    pub id: Option<String>,
    pub email: Option<String>,
    pub informations: Option<StaffMemberInformationsAggregation>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaffMemberInformationsAggregation {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub professional: Option<UserProfessionalAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfessionalAggregation {
    pub name: Option<String>,
    pub logo: Option<LogoAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogoAggregation {
    pub id: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListRolesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "role_not_found")]
    RoleNotFound,
    Default(String),
}

impl ResponseError for GetListRolesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListRolesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListRolesError::RoleNotFound => HttpResponse::Conflict().body("role_not_found"),
            GetListRolesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}