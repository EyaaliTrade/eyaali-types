use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductInventoriesBody {
    pub menu: String,
    pub vendor_id: Option<String>,
    pub product_name: Option<String>,
    pub barcode: Option<String>,
    pub availabilities: Option<Vec<String>>,
    pub page_number: i32,
    pub page_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductInventoriesResult {
    pub total: i32,
    pub list: Vec<ProductInventoryAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductInventoryAggregation {
    pub product: Option<String>,
    pub variants: Option<Vec<InventoryVariantAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryVariantAggregation {
    pub variant: Option<String>,
    pub availability: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListProductInventoriesError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetListProductInventoriesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListProductInventoriesError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListProductInventoriesError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetListProductInventoriesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
