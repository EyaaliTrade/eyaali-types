use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListGiftCardsBody {
    pub menu: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListGiftCardsResult {
    pub list: Vec<GiftCardAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardAggregation {
    pub id: Option<String>,
    pub names: Option<Vec<GiftCardNameAggregation>>,
    pub descriptions: Option<Vec<GiftCardDescriptionAggregation>>,
    pub price: Option<GiftCardPriceAggregation>,
    pub customers: Option<Vec<GiftCardCustomerIdAggregation>>,
    pub groups: Option<Vec<GiftCardGroupIdAggregation>>,
    pub codes: Option<Vec<GiftCardCodeAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardPriceAggregation {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardCustomerIdAggregation {
    pub customer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardGroupIdAggregation {
    pub group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardCodeAggregation {
    pub email: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListGiftCardsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListGiftCardsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListGiftCardsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListGiftCardsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
