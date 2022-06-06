use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateCompanySiteBody {
    pub company: Option<String>,
    pub menu: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<CreateCompanySiteNameBody>>,
    pub descriptions: Option<Vec<CreateCompanySiteDescriptionBody>>,
    pub address: Option<CreateCompanySiteAddressBody>,
    pub phone: Option<CompanySitePhoneBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCompanySiteNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCompanySiteDescriptionBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCompanySiteAddressBody {
    pub road_names: Option<Vec<CreateAddressRoadNameBody>>,
    pub postal_code: Option<i32>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub location: Option<AddressLocationBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAddressRoadNameBody {
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
pub struct CreateCompanySiteResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateCompanySiteError {
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    #[display(fmt = "identifier_is_not_alphabectic")]
    IdentifierIsNotAplhabetic,
    Default(String),
}

impl ResponseError for CreateCompanySiteError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateCompanySiteError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            CreateCompanySiteError::IdentifierIsNotAplhabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabectic")
            }
            CreateCompanySiteError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
