use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListRoadKindsBody {
    pub language_code: String,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListRoadKindsResult {
    pub list: Vec<RoadKindAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoadKindAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListRoadKindsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListRoadKindsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListRoadKindsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListRoadKindsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
