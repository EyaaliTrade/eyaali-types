use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetListCompanySettingsBody {
    pub is_ready: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCompanySettingsResult {
    pub list: Vec<CompanySettingsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CompanySettingsAggregation {
    pub id: Option<String>,
    pub is_ready: Option<bool>,
    pub is_on_promise: Option<bool>,
    pub urls: Option<CompanySettingsUrlsAggregation>,
    pub server: Option<CompanySettingsServerAggregation>,
    pub mailer: Option<CompanySettingsMailerAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanySettingsUrlsAggregation {
    pub provider: Option<String>,
    pub root_domain: Option<String>,
    pub api_sub_domain: Option<String>,
    pub admin_sub_domain: Option<String>,
    pub manager_sub_domain: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanySettingsServerAggregation {
    pub provider: Option<String>,
    pub ip_address: Option<String>,
    pub ssh_user: Option<String>,
    pub ssh_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanySettingsMailerAggregation {
    pub host: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub no_reply_email: Option<String>,
    pub company_name: Option<String>,
    pub company_logo_url: Option<String>,
    pub company_color: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListCompanySettingsError {
    Default(String),
}

impl ResponseError for GetListCompanySettingsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCompanySettingsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
