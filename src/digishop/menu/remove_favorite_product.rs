use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RemoveFavoriteProductBody {
    pub user: Option<String>,
    pub device: Option<String>,
    pub product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveFavoriteProductResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum RemoveFavoriteProductError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    #[display(fmt = "favorite_product_not_found")]
    FavoriteProductNotFound,
    Default(String),
}

impl ResponseError for RemoveFavoriteProductError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RemoveFavoriteProductError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            RemoveFavoriteProductError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            RemoveFavoriteProductError::ProductNotFound => {
                HttpResponse::Conflict().body("product_not_found")
            }
            RemoveFavoriteProductError::FavoriteProductNotFound => {
                HttpResponse::Conflict().body("favorite_product_not_found")
            }
            RemoveFavoriteProductError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
