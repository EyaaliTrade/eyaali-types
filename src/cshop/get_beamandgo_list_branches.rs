use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBeamAndGoListBranchesBody {
    pub product_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBeamAndGoListBranchesResult {
    pub list: Vec<BeamAndGoBranchAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeamAndGoBranchAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetBeamAndGoListBranchesError {
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetBeamAndGoListBranchesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetBeamAndGoListBranchesError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetBeamAndGoListBranchesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
