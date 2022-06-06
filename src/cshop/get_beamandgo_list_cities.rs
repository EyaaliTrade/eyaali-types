use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBeamAndGoListCitiesBody {
    pub province_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBeamAndGoListCitiesResult {
    pub list: Vec<BeamAndGoCityAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeamAndGoCityAggregation {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetBeamAndGoListCitiesError {
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetBeamAndGoListCitiesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetBeamAndGoListCitiesError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetBeamAndGoListCitiesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
