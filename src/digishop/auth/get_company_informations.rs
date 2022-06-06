use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCompanyInformationsBody {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCompanyInformationsResult {
    pub company: CompanyAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
    pub phone: Option<PhoneAggregation>,
    pub email: Option<String>,
    pub descriptions: Option<Vec<DescriptionAggregation>>,
    pub website: Option<String>,
    pub address: Option<AddressAggregation>,
    pub socials: Option<Vec<SocialAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhoneAggregation {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocialAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressAggregation {
    pub id: Option<String>,
    pub road_names: Option<Vec<AddressRoadNameAggregation>>,
    pub postal_code: Option<i32>,
    pub country: Option<String>,
    pub city: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressRoadNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetCompanyInformationsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "company_not_found")]
    CompanyNotFound,
    Default(String),
}

impl ResponseError for GetCompanyInformationsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetCompanyInformationsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetCompanyInformationsError::CompanyNotFound => {
                HttpResponse::Conflict().body("company_not_found")
            }
            GetCompanyInformationsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
