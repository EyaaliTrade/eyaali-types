use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateStaffMemberBody {
    pub menu: String,
    pub kind: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name:  String,
    pub is_assigned: Option<bool>,
    pub language_code: Option<String>,
    pub activation_status: Option<String>,
    pub role: Option<String>,
    pub company: Option<StaffMemberCompanyBody>,
    pub is_vendor: Option<bool>,
    pub professional: Option<UserProfessionalBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfessionalBody {
    pub name: Option<String>,
    pub logo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaffMemberCompanyBody {
    pub id: Option<String>,
    pub name: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  CreateStaffMemberResult {
    pub id: String
}

#[derive(Debug, Display)]
pub enum CreateStaffMemberError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "error_hashing_password")]
    HashPassword,
    #[display(fmt = "email_already_exists")]
    EmailExists,
    Default(String),
}

impl ResponseError for CreateStaffMemberError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateStaffMemberError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            },
            CreateStaffMemberError::HashPassword =>  HttpResponse::InternalServerError().body("error_hashing_password"),
            CreateStaffMemberError::EmailExists => HttpResponse::Conflict().body("email_already_exists"),
            CreateStaffMemberError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}