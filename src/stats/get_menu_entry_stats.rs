use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuEntryStatsBody {
    pub menu: String,
    pub year: i32,
    pub month: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuEntryStatsResult {
    pub list: Vec<MenuEntryStatsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuEntryStatsAggregation {
    pub day_of_week: Option<i32>,
    pub count: Option<i32>,
    pub details: Option<Vec<DayOfWeekDetailsAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DayOfWeekDetailsAggregation {
    pub day: Option<i32>,
    pub count: Option<i32>,
}

#[derive(Debug, Display)]
pub enum GetMenuEntryStatsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_month")]
    InvalidMonth,
    Default(String),
}

impl ResponseError for GetMenuEntryStatsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuEntryStatsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuEntryStatsError::InvalidMonth => HttpResponse::Conflict().body("invalid_month"),
            GetMenuEntryStatsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
