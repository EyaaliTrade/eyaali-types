use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRoleBody {
  pub id: String,
  pub kind: Option<String>,
  pub permissions: Option<Vec<UpdateRolePermissionBody>>,
  pub staff: Option<Vec<UpdateRoleStaffBody>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRolePermissionBody {
    pub permission: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRoleStaffBody {
    pub user: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRoleResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateRoleError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "role_not_found")]
    RoleNotFound,
    Default(String),
}

impl ResponseError for UpdateRoleError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateRoleError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateRoleError::RoleNotFound => HttpResponse::NotFound().body("role_not_found"),
            UpdateRoleError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}

