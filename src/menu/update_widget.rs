use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateWidgetBody{
    pub id: String,
    pub kind: Option<String>,
    pub area: Option<String>,
    pub order: Option<i32>,
    pub names: Option<Vec<UpdateWidgetNameBody>>,
    pub styles: Option<Vec<UpdateWidgetStyleBody>>,
    pub content: Option<UpdateWidgetContentBody>,
    pub is_visible: Option<bool>,
    pub is_deleted: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateWidgetStyleBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateWidgetNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateWidgetContentBody {
    pub text_content: Option<Vec<UpdateWidgetTextContentBody>>,
    pub media: Option<UpdateWidgetMediaContentBody>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateWidgetTextContentBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateWidgetMediaContentBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateWidgetResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateWidgetError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "widget_not_found")]
    WidgetNotFound,
    Default(String),
}

impl ResponseError for UpdateWidgetError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateWidgetError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateWidgetError::WidgetNotFound => {
                HttpResponse::Conflict().body("widget_not_found")
            }
            UpdateWidgetError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}