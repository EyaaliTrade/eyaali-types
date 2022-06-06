use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserByEmailBody {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserByEmailResult {
    pub user: Option<UserAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAggregation {
    pub id: Option<String>,
    pub email: Option<String>,
    pub kind: Option<String>,
    pub password: Option<String>,
    pub informations: Option<UserInformationsAggregation>,
    pub contact: Option<UserContactAggregation>,
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
pub struct CompanyLogoAggregation {
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
    pub road_name: Option<String>,
    pub postal_code: Option<i32>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub location: Option<AddressLocationAggregation>,
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

#[derive(Debug, Display)]
pub enum GetUserByEmailError {
    Default(String),
}

impl ResponseError for GetUserByEmailError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetUserByEmailError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
