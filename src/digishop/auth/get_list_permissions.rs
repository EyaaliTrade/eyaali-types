use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListPermissionsBody {
    pub language_code:  Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListPermissionsResult {
    pub list: Vec<PermissionAggregation>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionAggregation {
    pub id: Option<String>,
    pub kind: Option<String>,
    pub names: Option<Vec<PermissionNameAggregation>>,
    pub descriptions: Option<Vec<PermissionDescriptionAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListPermissionsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListPermissionsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListPermissionsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListPermissionsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}