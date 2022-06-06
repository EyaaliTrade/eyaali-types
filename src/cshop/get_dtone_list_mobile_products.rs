use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDTOneListMobileProductsBody {
    pub operator_id: i32,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDTOneListMobileProductsResult {
    pub list: Vec<DTOneProductAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DTOneProductAggregation {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub operator_name: Option<String>,
    pub destination: Option<AmountUnitAggregation>,
    pub source: Option<AmountUnitAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AmountUnitAggregation {
    pub amount: Option<f64>,
    pub unit: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetDTOneListMobileProductsError {
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetDTOneListMobileProductsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetDTOneListMobileProductsError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetDTOneListMobileProductsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
