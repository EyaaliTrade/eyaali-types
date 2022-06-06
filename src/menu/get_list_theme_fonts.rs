use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListThemeFontsBody {
    pub company: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListThemeFontsResult {
    pub list: Vec<ThemeFontAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeFontAggregation {
    pub id: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListThemeFontsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListThemeFontsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListThemeFontsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListThemeFontsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
