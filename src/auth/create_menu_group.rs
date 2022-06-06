use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateMenuGroupBody {
    pub menu: String,
    pub names: Option<Vec<CreateMenuGroupNameBody>>,
    pub descriptions: Option<Vec<CreateMenuGroupDescriptionBody>>,
    pub customers: Option<Vec<MenuGroupCustomerIdBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuGroupNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuGroupDescriptionBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuGroupCustomerIdBody {
    pub customer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuGroupResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateMenuGroupError {
    Default(String),
}

impl ResponseError for CreateMenuGroupError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateMenuGroupError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
