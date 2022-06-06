use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateGiftCardBody {
    pub menu: String,
    pub names: Option<Vec<CreateGiftCardNameBody>>,
    pub descriptions: Option<Vec<CreateGiftCardDescriptionBody>>,
    pub price: Option<PriceBody>,
    pub customers: Option<Vec<GiftCardCustomerIdBody>>,
    pub groups: Option<Vec<GiftCardGroupIdBody>>,
    pub codes: Option<Vec<GiftCardCodeBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateGiftCardNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateGiftCardDescriptionBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceBody {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardCustomerIdBody {
    pub customer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardGroupIdBody {
    pub group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardCodeBody {
    pub customer: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateGiftCardResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateGiftCardError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "code_already_exists")]
    CodeExists,
    #[display(fmt = "invalid_request")]
    InvalidRequest,
    Default(String),
}

impl ResponseError for CreateGiftCardError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateGiftCardError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            CreateGiftCardError::CodeExists => HttpResponse::Conflict().body("code_already_exists"),
            CreateGiftCardError::InvalidRequest => {
                HttpResponse::BadRequest().body("invalid_request")
            }
            CreateGiftCardError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
