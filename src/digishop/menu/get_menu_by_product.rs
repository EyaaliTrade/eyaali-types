use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuByProductBody {
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuByProductResult {
    pub menu: String,
}

#[derive(Debug, Display)]
pub enum GetMenuByProductError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetMenuByProductError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuByProductError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuByProductError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
