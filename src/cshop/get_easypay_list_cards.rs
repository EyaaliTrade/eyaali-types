use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetEasyPayListCardsBody {
    pub cat_id: String,
    pub region_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetEasyPayListCardsResult {
    pub list: Vec<EasyPayCardAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EasyPayCardAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetEasyPayListCardsError {
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetEasyPayListCardsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetEasyPayListCardsError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetEasyPayListCardsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
