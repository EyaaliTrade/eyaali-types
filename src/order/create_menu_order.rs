use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateMenuOrderBody {
    pub menu: String,
    pub device: String,
    pub id: Option<String>,
    pub kind: Option<String>,
    pub payment_method: Option<String>,
    pub customer_informations: Option<CustomerInformationsBody>,
    pub customer_notes: Option<String>,
    pub fcm_devices: Vec<FcmDeviceBody>,
    pub prep_time: Option<i32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerInformationsBody {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub address: Option<CreateCustomerAddressBody>,
    pub phone: Option<CustomerPhoneBody>,
    pub position: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCustomerAddressBody {
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
pub struct CustomerPhoneBody {
    pub country_code: Option<i32>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FcmDeviceBody {
    pub device: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMenuOrderResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateMenuOrderError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "reference_already_exists")]
    ReferenceExists,
    #[display(fmt = "cart_is_empty")]
    CartIsEmpty,
    Default(String),
}

impl ResponseError for CreateMenuOrderError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateMenuOrderError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateMenuOrderError::ReferenceExists => {
                HttpResponse::Conflict().body("reference_already_exists")
            }
            CreateMenuOrderError::CartIsEmpty => HttpResponse::Gone().body("cart_is_empty"),
            CreateMenuOrderError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
