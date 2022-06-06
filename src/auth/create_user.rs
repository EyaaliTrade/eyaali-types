use actix_web::{HttpResponse, ResponseError};
use chrono::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateUserBody {
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub kind: String,
    pub account_activation: Option<AccountActivationBody>,
    pub account_verification: Option<AccountVerificationBody>,
    pub informations: Option<UserInformationsBody>,
    pub contact: Option<UserContactBody>,
    pub account_settings: Option<AccountSettingsBody>,
    pub company: Option<CreateCompanyBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountActivationBody {
    pub activation_date: Option<DateTime<Utc>>,
    pub activation_status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountVerificationBody {
    pub is_verified: Option<bool>,
    pub verification_code: Option<String>,
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
    pub address: Option<CreateUserAddressBody>,
    pub phone: Option<UserPhoneBody>,
    pub socials: Option<Vec<UserSocialBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserAddressBody {
    pub road_names: Option<Vec<CreateAddressRoadNameBody>>,
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
pub struct CreateCompanyBody {
    pub kind: Option<String>,
    pub domain: Option<String>,
    pub short_names: Option<Vec<CreateCompanyShortNameBody>>,
    pub long_names: Option<Vec<CreateCompanyLongNameBody>>,
    pub descriptions: Option<Vec<CreateCompanyDescriptionBody>>,
    pub activities: Option<Vec<CompanyActivityBody>>,
    pub fiscal: Option<CreateCompanyFiscalBody>,
    pub logo: Option<String>,
    pub address: Option<CreateCompanyAddressBody>,
    pub website: Option<String>,
    pub socials: Option<Vec<CompanySocialBody>>,
    pub contacts: Option<Vec<CompanyContactBody>>,
    pub characteristics: Option<Vec<CompanyCharacteristicBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCompanyShortNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCompanyLongNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCompanyDescriptionBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyActivityBody {
    pub activity: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCompanyFiscalBody {
    pub corporate_names: Option<Vec<CreateCompanyCorporateNameBody>>,
    pub legal_form: Option<String>,
    pub matriculation: Option<String>,
    pub tax_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCompanyCorporateNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCompanyAddressBody {
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
pub struct CreateUserResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateUserError {
    #[display(fmt = "error_hashing_password")]
    HashPassword,
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

impl ResponseError for CreateUserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateUserError::HashPassword => {
                HttpResponse::InternalServerError().body("error_hashing_password")
            }
            CreateUserError::EmailExists => HttpResponse::Conflict().body("email_already_exists"),
            CreateUserError::DomainIsNotAplhabetic => {
                HttpResponse::NotAcceptable().body("domain_is_not_alphabectic")
            }
            CreateUserError::DomainContainsNumber => {
                HttpResponse::NotAcceptable().body("domain_contains_number")
            }
            CreateUserError::DomainIsReserved => {
                HttpResponse::NotAcceptable().body("domain_is_reserved")
            }
            CreateUserError::DomainIsTooShort => {
                HttpResponse::NotAcceptable().body("domain_is_too_short")
            }
            CreateUserError::DomainExists => {
                HttpResponse::UnprocessableEntity().body("domain_already_exists")
            }
            CreateUserError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
