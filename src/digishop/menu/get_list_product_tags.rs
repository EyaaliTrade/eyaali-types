use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductTagsBody {
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListProductTagsResult {
    pub list: Vec<ProductTagAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductTagAggregation {
    pub character: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Display)]
pub enum GetListProductTagsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListProductTagsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListProductTagsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListProductTagsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
