use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductEntryStatsBody {
    pub menu: String,
    pub year: i32,
    pub month: i32,
    pub page_number: i32,
    pub page_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductEntryStatsResult {
    pub list: Vec<ProductEntryStatsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductEntryStatsAggregation {
    pub id: Option<String>,
    pub current_month_count: Option<i32>,
    pub previous_month_count: Option<i32>,
    pub difference_count: Option<i32>,
}

#[derive(Debug, Display)]
pub enum GetProductEntryStatsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_month")]
    InvalidMonth,
    Default(String),
}

impl ResponseError for GetProductEntryStatsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductEntryStatsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductEntryStatsError::InvalidMonth => {
                HttpResponse::Conflict().body("invalid_month")
            }
            GetProductEntryStatsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
