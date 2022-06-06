use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct VerifyServerRequirementsBody {
    pub api_sub_domain: String,
    pub admin_sub_domain: String,
    pub manager_sub_domain: String,
    pub ip_address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyServerRequirementsResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum VerifyServerRequirementsError {
    #[display(fmt = "invalid_domain")]
    InvalidDomain,
    #[display(fmt = "invalid_address_ip")]
    InvalidAddressIP,
    #[display(fmt = "not_matching_address_ip")]
    NotMatchingAddressIP,
    Default(String),
}

impl ResponseError for VerifyServerRequirementsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            VerifyServerRequirementsError::InvalidDomain => {
                HttpResponse::Conflict().body("invalid_domain")
            }
            VerifyServerRequirementsError::InvalidAddressIP => {
                HttpResponse::Conflict().body("invalid_address_ip")
            }
            VerifyServerRequirementsError::NotMatchingAddressIP => {
                HttpResponse::Conflict().body("not_matching_address_ip")
            }
            VerifyServerRequirementsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
