use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCompanyOwnerBody {
    pub company: String,
    pub kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCompanyOwnerResult {
    pub owner: Option<CompanyOwnerAggregation>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyOwnerAggregation {
    pub id: Option<String>,
    pub email: Option<String>,
    pub informations: Option<OwnerInformationsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OwnerInformationsAggregation {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<String>,
    pub gender: Option<String>,
    pub civil_status: Option<String>,
    pub picture: Option<UserPictureAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyLogoAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetCompanyOwnerError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetCompanyOwnerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCompanyOwnerError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCompanyOwnerError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
