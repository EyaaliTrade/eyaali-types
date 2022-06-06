use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListTagsByMenuBody {
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListTagsByMenuResult {
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagsByMenuAggregation {
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Display)]
pub enum GetListTagsByMenuError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListTagsByMenuError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListTagsByMenuError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListTagsByMenuError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
