use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCountMenuOrdersBody {
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCountMenuOrdersResult {
    pub count_all: Option<i32>,
    pub count_new_paid: Option<i32>,
    pub count_new_unpaid: Option<i32>,
    pub count_in_process: Option<i32>,
    pub count_processed: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuOrderCountAggregation {
    pub count_all: Option<i32>,
    pub count_new_paid: Option<i32>,
    pub count_new_unpaid: Option<i32>,
    pub count_in_process: Option<i32>,
    pub count_processed: Option<i32>,
}

#[derive(Debug, Display)]
pub enum GetCountMenuOrdersError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetCountMenuOrdersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCountMenuOrdersError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCountMenuOrdersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
