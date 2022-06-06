use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct SetProductSeoDataBody {
    pub product: String,
    pub titles: Option<Vec<SetProductTitleBody>>,
    pub descriptions: Option<Vec<SetProductDescriptionBody>>,
    pub identifier: Option<String>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetProductDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetProductTitleBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetProductSeoDataResult {
    pub success: bool,
}


#[derive(Debug, Display)]
pub enum SetProductSeoDataError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    #[display(fmt = "identifier_is_not_alphabetic")]
    IdentifierIsNotAlphabetic,
    Default(String),
}

impl ResponseError for SetProductSeoDataError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SetProductSeoDataError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            SetProductSeoDataError::ProductNotFound => {
                HttpResponse::NotFound().body("product_not_found")
            }
            SetProductSeoDataError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            SetProductSeoDataError::IdentifierIsNotAlphabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabetic")
            }
            SetProductSeoDataError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
