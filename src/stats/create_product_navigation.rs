use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateProductNavigationBody {
    pub company: String,
    pub customer: Option<String>,
    pub device: Option<String>,
    pub menu: String,
    pub product: String,
    pub kind: String,
    pub navigation_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProductNavigationResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum CreateProductNavigationError {
    Default(String),
}

impl ResponseError for CreateProductNavigationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateProductNavigationError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
