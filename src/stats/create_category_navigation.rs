use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateCategoryNavigationBody {
    pub company: String,
    pub customer: Option<String>,
    pub device: Option<String>,
    pub menu: String,
    pub category: String,
    pub kind: String,
    pub navigation_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCategoryNavigationResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum CreateCategoryNavigationError {
    Default(String),
}

impl ResponseError for CreateCategoryNavigationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateCategoryNavigationError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
