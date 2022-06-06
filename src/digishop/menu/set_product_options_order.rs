use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SetProductOptionsOrderBody {
    pub product: String,
    pub options: Option<Vec<ProductOptionOrderBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductOptionOrderBody {
    pub id: String,
    pub order: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetProductOptionsOrderResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum SetProductOptionsOrderError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for SetProductOptionsOrderError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SetProductOptionsOrderError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SetProductOptionsOrderError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
