use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetWidgetDetailsBody {
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetWidgetDetailsResult {
    pub widget: WidgetAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetAggregation {
    pub id: Option<String>,
    pub kind: Option<String>,
    pub area: Option<String>,
    pub names: Option<Vec<WidgetNameAggregation>>,
    pub styles: Option<Vec<WidgetStyleAggregation>>,
    pub order: Option<i32>,
    pub content: Option<WidgetContentAggregation>,
    pub is_visible: Option<bool>,
    pub is_deleted: Option<bool>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetStyleAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetContentAggregation {
    pub text_content: Option<Vec<WidgetTextContentAggregation>>,
    pub media: Option<WidgetMediaAggregation>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetTextContentAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetMediaAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Display)]
pub enum GetWidgetDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "widget_not_found")]
    WidgetNotFound,
    Default(String),
}

impl ResponseError for GetWidgetDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetWidgetDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetWidgetDetailsError::WidgetNotFound => HttpResponse::Conflict().body("widget_not_found"),
            GetWidgetDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}


