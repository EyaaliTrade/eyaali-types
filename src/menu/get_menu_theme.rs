use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuThemeBody {
    pub menu: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMenuThemeResult {
    pub menu_theme: Option<MenuThemeAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuThemeAggregation {
    pub id: Option<String>,
    pub menu: Option<String>,
    pub background_style: Option<Vec<BackgroundStyleAggregation>>,
    pub font_styles: Option<Vec<FontStyleAggregation>>,
    pub color_palette: Option<Vec<ColorAggregation>>,
    pub title_background: Option<Vec<TitleBackgroundAggregation>>,
    pub buttons_style: Option<Vec<MenuThemeButtonAggregation>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FontStyleAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuThemeButtonAggregation {
    pub kind: Option<String>,
    pub background_image: Option<String>,
    pub styles: Option<Vec<BackgroundStyleAggregation>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ButtonStyleAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackgroundStyleAggregation {
    pub kind: Option<String>,
    pub value: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TitleBackgroundAggregation {
    pub kind: Option<String>,
    pub value: Option<String>,
    pub font_size: Option<i32>
}

#[derive(Debug, Display)]
pub enum GetMenuThemeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_theme_not_found")]
    MenuThemeNotFound,
    Default(String),
}

impl ResponseError for GetMenuThemeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetMenuThemeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetMenuThemeError::MenuThemeNotFound => {
                HttpResponse::Conflict().body("menu_theme_not_found")
            }
            GetMenuThemeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
