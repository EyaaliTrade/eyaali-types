use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RemoveProductOptionBody {
    pub product: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveProductOptionResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum RemoveProductOptionError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    #[display(fmt = "option_not_found")]
    OptionNotFound,
    Default(String),
}

impl ResponseError for RemoveProductOptionError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RemoveProductOptionError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            RemoveProductOptionError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            RemoveProductOptionError::OptionNotFound => {
                HttpResponse::Conflict().body("option_not_found")
            }
            RemoveProductOptionError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
