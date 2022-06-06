use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCustomerOrdersBody {
    pub user: String,
    pub menu: String,
    pub vendor_id: Option<String>,
    pub vendor_products_id: Option<Vec<String>>,
    pub reference: Option<String>,
    pub sorting: Option<String>,
    pub order_status: Option<String>,
    pub page_number: i32,
    pub page_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCustomerOrdersResult {
    pub list: Vec<CustomerOrderAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerOrderAggregation {
    pub id: Option<String>,
    pub reference: Option<String>,
    pub order_status: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListCustomerOrdersError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetListCustomerOrdersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCustomerOrdersError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListCustomerOrdersError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetListCustomerOrdersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
