use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListGendersBody {
    pub language_code: String,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListGendersResult {
    pub list: Vec<GenderAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenderAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListGendersError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListGendersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListGendersError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListGendersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
