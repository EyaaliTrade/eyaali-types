use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuProductsBody {
    pub language_code: String,
    pub menus: Vec<String>,
    pub categories: Vec<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuProductsResult {
    pub list: Vec<MenuProductAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuProductAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
    pub picture: Option<MenuProductPictureAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuProductPictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListMenuProductsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListMenuProductsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMenuProductsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListMenuProductsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
