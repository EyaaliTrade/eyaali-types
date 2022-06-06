use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryEntryStatsBody {
    pub menu: String,
    pub year: i32,
    pub month: i32,
    pub page_number: i32,
    pub page_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCategoryEntryStatsResult {
    pub list: Vec<CategoryEntryStatsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryEntryStatsAggregation {
    pub id: Option<String>,
    pub current_month_count: Option<i32>,
    pub previous_month_count: Option<i32>,
    pub difference_count: Option<i32>,
}

#[derive(Debug, Display)]
pub enum GetCategoryEntryStatsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_month")]
    InvalidMonth,
    Default(String),
}

impl ResponseError for GetCategoryEntryStatsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCategoryEntryStatsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCategoryEntryStatsError::InvalidMonth => {
                HttpResponse::Conflict().body("invalid_month")
            }
            GetCategoryEntryStatsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
