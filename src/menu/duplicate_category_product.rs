use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct DuplicateCateroyProductBody {
    pub template_id: String,
    pub template_product: String,
    pub category: String,
    pub languages: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DuplicateCateroyProductResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum DuplicateCateroyProductError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "template_product_not_found")]
    TemplateProductNotFound,
    Default(String),
}

impl ResponseError for DuplicateCateroyProductError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DuplicateCateroyProductError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            DuplicateCateroyProductError::TemplateProductNotFound => {
                HttpResponse::NotFound().body("template_product_not_found")
            }
            DuplicateCateroyProductError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}

