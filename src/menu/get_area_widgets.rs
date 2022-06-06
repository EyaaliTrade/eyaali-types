use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAreaWidgetsBody {
    pub area: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAreaWidgetsResult {
    pub widgets: Option<Vec<WidgetAggregation>>,
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
pub enum GetAreaWidgetsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetAreaWidgetsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetAreaWidgetsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetAreaWidgetsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}

