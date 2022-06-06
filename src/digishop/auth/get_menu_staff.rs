use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuStaffBody {
    pub menu: String,
    pub kind: String,
    pub sorting: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuStaffResult {
    pub list: Vec<StaffMemberAggregation>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaffMemberAggregation {
    pub id: Option<String>,
    pub email: Option<String>,
    pub informations: Option<StaffMemberInformationsAggregation>,
    pub role: Option<RoleAggregation>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleAggregation {
    pub id: Option<String>,
    pub kind: Option<String>
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
pub enum GetMenuStaffError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_not_found")]
    MenuNotFound,
    Default(String),
}

impl ResponseError for GetMenuStaffError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuStaffError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuStaffError::MenuNotFound => HttpResponse::Conflict().body("menu_not_found"),
            GetMenuStaffError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
