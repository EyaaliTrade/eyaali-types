use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCompanyActivitiesBody {
    pub language_code: String,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCompanyActivitiesResult {
    pub list: Vec<ActivityAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListCompanyActivitiesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListCompanyActivitiesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCompanyActivitiesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListCompanyActivitiesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
