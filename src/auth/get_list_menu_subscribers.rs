use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuSubscribersBody {
    pub menu: String,
    pub origin: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuSubscribersResult {
    pub list: Vec<MenuSubscriberAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuSubscriberAggregation {
    pub id: Option<String>,
    pub email: Option<String>,
    pub origin: Option<String>
}

#[derive(Debug, Display)]
pub enum GetListMenuSubscribersError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListMenuSubscribersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMenuSubscribersError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListMenuSubscribersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
