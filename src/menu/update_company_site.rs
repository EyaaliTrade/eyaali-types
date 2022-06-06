use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateCompanySiteBody {
    pub id: String,
    pub company: Option<String>,
    pub menu: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<UpdateCompanySiteNameBody>>,
    pub descriptions: Option<Vec<UpdateCompanySiteDescriptionBody>>,
    pub address: Option<UpdateCompanySiteAddressBody>,
    pub phone: Option<CompanySitePhoneBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCompanySiteNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCompanySiteDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCompanySiteAddressBody {
    pub id: Option<String>,
    pub road_names: Option<Vec<UpdateAddressRoadNameBody>>,
    pub postal_code: Option<i32>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub location: Option<AddressLocationBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAddressRoadNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressLocationBody {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanySitePhoneBody {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCompanySiteResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateCompanySiteError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "company_site_not_found")]
    CompanySiteNotFound,
    #[display(fmt = "identifier_is_not_alphabectic")]
    IdentifierIsNotAplhabetic,
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    Default(String),
}

impl ResponseError for UpdateCompanySiteError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateCompanySiteError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateCompanySiteError::CompanySiteNotFound => {
                HttpResponse::Conflict().body("company_site_not_found")
            }
            UpdateCompanySiteError::IdentifierIsNotAplhabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabectic")
            }
            UpdateCompanySiteError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            UpdateCompanySiteError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
