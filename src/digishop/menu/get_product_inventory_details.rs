use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductInventoryDetailsBody {
    pub product: String,
    pub variant: Option<String>,
    pub site: Option<String>,
    pub page_number: i32,
    pub page_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductInventoryDetailsResult {
    pub list: Vec<ProductInventoryDetailsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductInventoryDetailsAggregation {
    pub id: Option<String>,
    pub date: Option<String>,
    pub cost: Option<f64>,
    pub available_quantity: Option<f64>,
    pub total_quantity: Option<f64>,
    pub stock_status: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetProductInventoryDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetProductInventoryDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductInventoryDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductInventoryDetailsError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetProductInventoryDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
