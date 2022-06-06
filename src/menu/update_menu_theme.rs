use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateMenuThemeBody {
    pub menu: String,
    pub background_style: Option<Vec<MenuThemeBackgroundStyleBody>>,
    pub font_styles: Option<Vec<MenuThemeFontStyleBody>>,
    pub color_palette: Option<Vec<MenuThemeColorPaletteBody>>,
    pub title_background: Option<Vec<MenuThemeTitleBackgroundBody>>,
    pub buttons_style: Option<Vec<MenuThemeButtonBody>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuThemeBackgroundStyleBody {
    pub kind: Option<String>,
    pub value: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuThemeFontStyleBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuThemeColorPaletteBody {
    pub kind: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuThemeTitleBackgroundBody {
    pub kind: Option<String>,
    pub value: Option<String>,
    pub font_size: Option<i32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductStyleBody {
    pub border_image: Option<String>,
    pub product: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuThemeButtonBody {
    pub kind: Option<String>,
    pub background_image: Option<String>,
    pub styles: Option<Vec<ButtonStyleBody>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ButtonStyleBody {
    pub kind: Option<String>,
    pub value: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMenuThemeResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateMenuThemeError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "menu_theme_not_found")]
    MenuThemeNotFound,
    Default(String),
}

impl ResponseError for UpdateMenuThemeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateMenuThemeError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateMenuThemeError::MenuThemeNotFound => {
                HttpResponse::Conflict().body("menu_theme_not_found")
            }
            UpdateMenuThemeError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
