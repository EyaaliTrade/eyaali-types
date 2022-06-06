use actix_web::{ResponseError, HttpResponse};

use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImportCustomersQuery {
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImportCustomersResult {
    pub success: bool
}

#[derive(Debug, Display)]
pub enum ImportCustomersError {
    Default(String),
}

impl ResponseError for ImportCustomersError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ImportCustomersError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}


#[derive(Debug, Deserialize)]
struct CustomerRecord {
    first_name: Option<String>,
    last_name: Option<String>,
    email: String,
    phone: Option<String>,
    country: Option<String>,
    city: Option<String>,
    address: Option<String>,
    building: Option<String>,
    floor: Option<String>,
    postal_code: Option<i32>,
}
