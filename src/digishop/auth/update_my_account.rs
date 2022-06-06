use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMyAccountBody {
    pub id: String,
    #[validate(email)]
    pub email: Option<String>,
    pub informations: Option<UserInformationsBody>,
    pub contact: Option<UserContactBody>,
    pub account_settings: Option<AccountSettingsBody>,
    pub company: Option<UpdateCompanyBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInformationsBody {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub civil_status: Option<String>,
    pub picture: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserContactBody {
    pub address: Option<UpdateMyAccountAddressBody>,
    pub phone: Option<UserPhoneBody>,
    pub socials: Option<Vec<UserSocialBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMyAccountAddressBody {
    pub id: Option<String>,
    pub road_names: Option<Vec<UpdateAddressRoadNameBody>>,
    pub postal_code: Option<i32>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub location: Option<AddressLocationBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPhoneBody {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSocialBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountSettingsBody {
    pub language: Option<String>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCompanyBody {
    pub id: Option<String>,
    pub short_names: Option<Vec<UpdateCompanyShortNameBody>>,
    pub long_names: Option<Vec<UpdateCompanyLongNameBody>>,
    pub descriptions: Option<Vec<UpdateCompanyDescriptionBody>>,
    pub activity: Option<String>,
    pub fiscal: Option<UpdateCompanyFiscalBody>,
    pub logo: Option<String>,
    pub address: Option<UpdateCompanyAddressBody>,
    pub website: Option<String>,
    pub socials: Option<Vec<CompanySocialBody>>,
    pub contacts: Option<Vec<CompanyContactBody>>,
    pub characteristics: Option<Vec<CompanyCharacteristicBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCompanyShortNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCompanyLongNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCompanyDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyActivityBody {
    pub activity: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCompanyFiscalBody {
    pub corporate_names: Option<Vec<UpdateCompanyCorporateNameBody>>,
    pub legal_form: Option<String>,
    pub matriculation: Option<String>,
    pub tax_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCompanyCorporateNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCompanyAddressBody {
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
pub struct CompanySocialBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyContactBody {
    pub level: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<CompanyPhoneBody>,
    pub email: Option<String>,
    pub position: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyPhoneBody {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyCharacteristicBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMyAccountResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMyAccountError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    #[display(fmt = "email_already_exists")]
    EmailExists,
    #[display(fmt = "domain_is_not_alphabectic")]
    DomainIsNotAplhabetic,
    #[display(fmt = "domain_contains_number")]
    DomainContainsNumber,
    #[display(fmt = "domain_is_reserved")]
    DomainIsReserved,
    #[display(fmt = "domain_is_too_short")]
    DomainIsTooShort,
    #[display(fmt = "domain_already_exists")]
    DomainExists,
    Default(String),
}

impl ResponseError for UpdateMyAccountError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMyAccountError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMyAccountError::UserNotFound => HttpResponse::Conflict().body("user_not_found"),
            UpdateMyAccountError::EmailExists => HttpResponse::Conflict().body("email_already_exists"),
            UpdateMyAccountError::DomainIsNotAplhabetic => {
                HttpResponse::NotAcceptable().body("domain_is_not_alphabectic")
            }
            UpdateMyAccountError::DomainContainsNumber => {
                HttpResponse::NotAcceptable().body("domain_contains_number")
            }
            UpdateMyAccountError::DomainIsReserved => {
                HttpResponse::NotAcceptable().body("domain_is_reserved")
            }
            UpdateMyAccountError::DomainIsTooShort => {
                HttpResponse::NotAcceptable().body("domain_is_too_short")
            }
            UpdateMyAccountError::DomainExists => {
                HttpResponse::UnprocessableEntity().body("domain_already_exists")
            }
            UpdateMyAccountError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
