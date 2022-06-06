use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AddFavoriteProductBody {
    pub user: Option<String>,
    pub device: Option<String>,
    pub menu: String,
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddFavoriteProductResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum AddFavoriteProductError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    #[display(fmt = "product_already_liked")]
    ProductAlreadyLiked,
    Default(String),
}

impl ResponseError for AddFavoriteProductError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AddFavoriteProductError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            AddFavoriteProductError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            AddFavoriteProductError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            AddFavoriteProductError::ProductAlreadyLiked => {
                HttpResponse::Conflict().body("product_already_liked")
            }
            AddFavoriteProductError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
