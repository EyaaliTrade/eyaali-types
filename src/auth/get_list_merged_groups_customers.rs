use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMergedGroupsCustomersBody {
    pub customers: Option<Vec<CustomerIdBody>>,
    pub group_customers: Option<Vec<CustomerIdBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerIdBody {
    pub customer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMergedGroupsCustomersResult {
    pub list: Vec<MergedGroupsCustomersAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergedGroupsCustomersAggregation {
    pub customer: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListMergedGroupsCustomersError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListMergedGroupsCustomersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMergedGroupsCustomersError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListMergedGroupsCustomersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
