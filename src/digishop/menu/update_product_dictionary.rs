use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductDictBody {
    pub id: String,
    pub names: Option<Vec<UpdateDictNameBody>>,
    pub descriptions: Option<Vec<UpdateDescriptionBody>>,
    pub options: Option<Vec<UpdateProductOptionBody>>,
    pub variants: Option<Vec<UpdateProductVariantBody>>,
    pub seo_data: Option<UpdateProductSeoBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductOptionBody {
    pub id: String,
    pub names: Option<Vec<UpdateDictNameBody>>,
    pub values: Option<Vec<UpdateProductOptionValueBody>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductOptionValueBody {
    pub id: String,
    pub names: Option<Vec<UpdateDictNameBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductVariantBody {
    pub id: String,
    pub names: Option<Vec<UpdateDictNameBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDictNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductSeoBody {
    pub id: String,
    pub titles: Option<Vec<UpdateDictNameBody>>,
    pub descriptions: Option<Vec<UpdateDescriptionBody>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProductDictResult {
    pub success: bool,
}


#[derive(Debug, Display)]
pub enum UpdateProductDictError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "product_not_found")]
    ProductNotFound,
    Default(String),
}

impl ResponseError for UpdateProductDictError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateProductDictError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateProductDictError::ProductNotFound => {
                HttpResponse::NotFound().body("product_not_found")
            }
            UpdateProductDictError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}