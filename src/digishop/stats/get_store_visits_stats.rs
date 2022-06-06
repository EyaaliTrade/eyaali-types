use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetStoreVisitsBody {
    pub menu: String,
    pub time_range: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeRangeBody {
    pub start: i64
    pub end: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetStoreVisitsResults {
    pub list: Vec<StoreVisitsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuEntryStatsAggregation {
    pub total_visits: Option<i32>,
    pub time_range: Option<TimeRangeAggregation>, //day in timestamp
    pub details: Option<Vec<VisitAggregation>>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeRangeAggregation {
    pub start: i64
    pub end: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisitAggregation {
    pub hour: i32,
    pub value: i32
}

#[derive(Debug, Display)]
pub enum GetStoreVisitsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_time_range")]
    InvalidTimeRange,
    Default(String),
}

impl ResponseError for GetStoreVisitsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetStoreVisitsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetStoreVisitsError::InvalidTimeRange => HttpResponse::Conflict().body("invalid_time_range"),
            GetStoreVisitsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
