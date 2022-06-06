use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuGroupsBody {
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListMenuGroupsResult {
    pub list: Vec<MenuGroupAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuGroupAggregation {
    pub id: Option<String>,
    pub names: Option<Vec<MenuGroupNameAggregation>>,
    pub descriptions: Option<Vec<MenuGroupDescriptionAggregation>>,
    pub customers_count: Option<i32>,
    pub customers_list: Option<Vec<PartialCustomerAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuGroupNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuGroupDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialCustomerAggregation {
    pub id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListMenuGroupsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListMenuGroupsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListMenuGroupsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListMenuGroupsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
