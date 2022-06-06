use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuProductOptionsBody {
    pub menu: String,
    pub categories: Option<Vec<String>>,
    pub language_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuProductOptionsResult {
    pub options: Vec<MenuProductOptionAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuProductOptionAggregation {
    pub kind: Option<String>,
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Display)]
pub enum GetMenuProductOptionsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetMenuProductOptionsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuProductOptionsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuProductOptionsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
