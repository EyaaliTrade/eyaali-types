use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuTypesBody {
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuTypesResult {
    pub list: Vec<MenuTypeAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuTypeAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub description: Option<MenuTypeDescriptionAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuTypeDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListMenuTypesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListMenuTypesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMenuTypesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListMenuTypesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
