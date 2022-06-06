use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListUserKindsBody {
    pub language_code: String,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListUserKindsResult {
    pub list: Vec<UserKindAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserKindAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListUserKindsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListUserKindsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListUserKindsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListUserKindsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
