use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMainMenuBody {
    pub company: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMainMenuResult {
    pub id: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetMainMenuError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "main_menu_not_found")]
    MainMenuNotFound,
    Default(String),
}

impl ResponseError for GetMainMenuError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMainMenuError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMainMenuError::MainMenuNotFound => {
                HttpResponse::Conflict().body("main_menu_not_found")
            }
            GetMainMenuError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
