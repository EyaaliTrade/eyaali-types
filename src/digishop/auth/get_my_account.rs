use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMyAccountBody {
    pub id: String,
    pub language_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMyAccountResult {
    pub account: UserAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAggregation {
    pub id: Option<String>,
    pub email: Option<String>,
    pub kind: Option<String>,
    pub account_activation: Option<AccountActivationAggregation>,
    pub informations: Option<UserInformationsAggregation>,
    pub contact: Option<UserContactAggregation>,
    pub account_settings: Option<AccountSettingsAggregation>,
    pub company: Option<CompanyAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountActivationAggregation {
    pub activation_date: Option<String>,
    pub activation_status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInformationsAggregation {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<String>,
    pub gender: Option<String>,
    pub civil_status: Option<String>,
    pub picture: Option<UserPictureAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserContactAggregation {
    pub address: Option<AddressAggregation>,
    pub phone: Option<UserPhoneAggregation>,
    pub socials: Option<Vec<UserSocialAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressAggregation {
    pub id: Option<String>,
    pub road_name: Option<Vec<AddressRoadNameAggregation>>,
    pub postal_code: Option<i32>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub location: Option<AddressLocationAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressRoadNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressLocationAggregation {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPhoneAggregation {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSocialAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountSettingsAggregation {
    pub language: Option<String>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyAggregation {
    pub id: Option<String>,
    pub kind: Option<String>,
    pub domain: Option<String>,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub description: Option<String>,
    pub activity: Option<String>,
    pub fiscal: Option<CompanyFiscalAggregation>,
    pub logo: Option<CompanyLogoAggregation>,
    pub address: Option<AddressAggregation>,
    pub website: Option<String>,
    pub socials: Option<Vec<CompanySocialAggregation>>,
    pub contacts: Option<Vec<CompanyContactAggregation>>,
    pub characteristics: Option<Vec<CompanyCharacteristicAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyFiscalAggregation {
    pub corporate_name: Option<String>,
    pub legal_form: Option<String>,
    pub matriculation: Option<String>,
    pub tax_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyLogoAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanySocialAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyContactAggregation {
    pub level: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<CompanyPhoneAggregation>,
    pub email: Option<String>,
    pub position: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyPhoneAggregation {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompanyCharacteristicAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetMyAccountError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "user_not_found")]
    UserNotFound,
    Default(String),
}

impl ResponseError for GetMyAccountError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMyAccountError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMyAccountError::UserNotFound => HttpResponse::Conflict().body("user_not_found"),
            GetMyAccountError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
