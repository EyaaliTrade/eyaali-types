use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetAccessPointSessionsBody {
    pub mac_address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccessPointSessionsResult {
    pub sessions: Vec<SessionAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SessionAggregation {
    pub user_id: String,
    pub access_point: String,
    pub characteristics: Option<Vec<SessionCharacteristic>>,
    pub session_type: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionCharacteristic {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetAccessPointSessionsError {
    #[display(fmt = "access_point_not_found")]
    AccessPointNotFound,
    Default(String),
}

impl ResponseError for GetAccessPointSessionsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAccessPointSessionsError::AccessPointNotFound => {
                HttpResponse::NotFound().body("access_point_not_found")
            }
            GetAccessPointSessionsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
