use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUnassignedStaffBody {
    pub menu: String,
    pub name: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUnassignedStaffResult {
    pub list: Vec<StaffMemberAggregation>
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
pub enum GetUnassignedStaffError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "Menu_not_found")]
    MenuNotFound,
    Default(String),
}

impl ResponseError for GetUnassignedStaffError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetUnassignedStaffError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetUnassignedStaffError::MenuNotFound => HttpResponse::Conflict().body("menu_not_found"),
            GetUnassignedStaffError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
