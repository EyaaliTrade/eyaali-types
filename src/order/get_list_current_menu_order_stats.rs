use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GetListCurrentMenuOrderStatsBody {
    pub menus: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCurrentMenuOrderStatsResult {
    pub all_count: Option<i32>,
    pub new_count: Option<i32>,
    pub pre_count: Option<i32>,
    pub old_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuOrderStatsAggregation {
    pub all_count: Option<i32>,
    pub new_count: Option<i32>,
    pub pre_count: Option<i32>,
    pub old_count: Option<i32>,
}

#[derive(Debug, Display)]
pub enum GetListCurrentMenuOrderStatsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListCurrentMenuOrderStatsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCurrentMenuOrderStatsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListCurrentMenuOrderStatsError::Default(error) => {
                HttpResponse::BadRequest().body(error)
            }
        }
    }
}
