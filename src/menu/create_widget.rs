use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateWidgetBody{
    pub kind: Option<String>,
    pub area: Option<String>,
    pub order: Option<i32>,
    pub names: Option<Vec<WidgetNameBody>>,
    pub styles: Option<Vec<WidgetStyleBody>>,
    pub content: Option<WidgetContentBody>,
    pub is_visible: Option<bool>,
    pub is_deleted: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetContentBody {
    pub text_content: Option<Vec<WidgetTextContentBody>>,
    pub media: Option<WidgetMediaContentBody>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetStyleBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetTextContentBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetMediaContentBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateWidgetResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateWidgetError {
    Default(String),
}

impl ResponseError for CreateWidgetError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateWidgetError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
