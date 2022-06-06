use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListSocialKindsBody {
    pub language_code: String,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetListSocialKindsResult {
    pub list: Vec<SocialKindAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocialKindAggregation {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetListSocialKindsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetListSocialKindsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetListSocialKindsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetListSocialKindsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
