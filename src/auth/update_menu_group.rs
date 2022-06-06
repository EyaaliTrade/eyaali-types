use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuGroupBody {
    pub id: String,
    pub menu: Option<String>,
    pub names: Option<Vec<UpdateMenuGroupNameBody>>,
    pub descriptions: Option<Vec<UpdateMenuGroupDescriptionBody>>,
    pub customers: Option<Vec<MenuGroupCustomerIdBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuGroupNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuGroupDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuGroupCustomerIdBody {
    pub customer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuGroupResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuGroupError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_group_not_found")]
    MenuGroupNotFound,
    Default(String),
}

impl ResponseError for UpdateMenuGroupError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuGroupError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuGroupError::MenuGroupNotFound => HttpResponse::Conflict().body("menu_group_not_found"),
            UpdateMenuGroupError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
