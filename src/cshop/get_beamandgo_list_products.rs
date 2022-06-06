use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBeamAndGoListProductsBody {
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBeamAndGoListProductsResult {
    pub list: Vec<BeamAndGoProductAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BeamAndGoProductAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub type_id: Option<i32>,
    pub unit_price: Option<i32>,
    pub unit_price_currency: Option<String>,
    pub image_url: Option<String>,
    pub min_quantity: Option<i32>,
    pub max_quantity: Option<i32>,
    pub stock_quantity: Option<i32>,
    pub is_branch_specific: Option<bool>,
    pub is_shipping_required: Option<bool>,
    pub is_birth_date_required:Option<bool>,
    pub account_sid: Option<String>,
    pub shipping_and_handling_fee: Option<i32>
}


#[derive(Debug, Display)]
pub enum GetBeamAndGoListProductsError {
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for GetBeamAndGoListProductsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetBeamAndGoListProductsError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            GetBeamAndGoListProductsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
