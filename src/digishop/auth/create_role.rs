use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRoleBody {
  pub kind: String,
  pub permissions: Option<Vec<CreateRolePermissionBody>>,
  pub staff: Option<Vec<CreateRoleStaffBody>>,
  pub menu: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRolePermissionBody {
    pub permission: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRoleStaffBody {
    pub user: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRoleResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateRoleError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for CreateRoleError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateRoleError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateRoleError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}

