use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SetCompanySettingsBody {
    pub is_on_promise: bool,
    pub admin_account: AdminAccountBody,
    pub urls: CompanySettingsUrlsBody,
    pub server: CompanySettingsServerBody,
    pub mailer: CompanySettingsMailerBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminAccountBody {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanySettingsUrlsBody {
    pub provider: String,
    pub root_domain: String,
    pub api_sub_domain: String,
    pub admin_sub_domain: String,
    pub manager_sub_domain: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanySettingsServerBody {
    pub provider: String,
    pub ip_address: String,
    pub ssh_user: String,
    pub ssh_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanySettingsMailerBody {
    pub host: String,
    pub username: String,
    pub password: String,
    pub no_reply_email: String,
    pub company_name: String,
    pub company_logo_url: String,
    pub company_color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetCompanySettingsResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum SetCompanySettingsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_domain")]
    InvalidDomain,
    #[display(fmt = "invalid_address_ip")]
    InvalidAddressIP,
    #[display(fmt = "not_matching_address_ip")]
    NotMatchingAddressIP,
    #[display(fmt = "deployment_failed")]
    DeploymentFailed,
    Default(String),
}

impl ResponseError for SetCompanySettingsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SetCompanySettingsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SetCompanySettingsError::InvalidDomain => {
                HttpResponse::Conflict().body("invalid_domain")
            }
            SetCompanySettingsError::InvalidAddressIP => {
                HttpResponse::Conflict().body("invalid_address_ip")
            }
            SetCompanySettingsError::NotMatchingAddressIP => {
                HttpResponse::Conflict().body("not_matching_address_ip")
            }
            SetCompanySettingsError::DeploymentFailed => {
                HttpResponse::InternalServerError().body("deployment_failed")
            }
            SetCompanySettingsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
