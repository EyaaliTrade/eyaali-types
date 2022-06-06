use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetEasyPayListCardClassesBody {
    pub card_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetEasyPayListCardClassesResult {
    pub list: Vec<EasyPayCardClassAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EasyPayCardClassAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub price: Option<f64>,
    pub retail_price: Option<f64>,
}

#[derive(Debug, Display)]
pub enum GetEasyPayListCardClassesError {
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetEasyPayListCardClassesError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetEasyPayListCardClassesError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetEasyPayListCardClassesError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
