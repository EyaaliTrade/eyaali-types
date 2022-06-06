use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCustomersByGroupsBody {
    pub groups: Option<Vec<GroupIdBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupIdBody {
    pub group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListCustomersByGroupsResult {
    pub list: Vec<CustomersByGroupsAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomersByGroupsAggregation {
    pub customer: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListCustomersByGroupsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListCustomersByGroupsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListCustomersByGroupsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListCustomersByGroupsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
