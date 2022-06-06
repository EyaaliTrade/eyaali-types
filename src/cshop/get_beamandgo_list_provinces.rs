use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBeamAndGoListProvincesResult {
    pub list: Vec<BeamAndGoProvinceAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeamAndGoProvinceAggregation {
    pub id: Option<i32>,
    pub country_region_id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetBeamAndGoListProvincesError {
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetBeamAndGoListProvincesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetBeamAndGoListProvincesError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetBeamAndGoListProvincesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
