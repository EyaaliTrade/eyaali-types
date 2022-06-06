//LIB
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListVendorByMenuBody {
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListVendorByMenuResult {
    pub vendors: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VendorByMenuAggregation {
    pub vendors: Option<Vec<String>>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VendorsAggregation {
    pub vendors: Vec<String>,
}

#[derive(Debug, Display)]
pub enum GetListVendorByMenuError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListVendorByMenuError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListVendorByMenuError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListVendorByMenuError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
