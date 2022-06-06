use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductDetailsBody {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub company: Option<String>,
    pub menu: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductDetailsResult {
    pub category_product: CategoryProductAggregation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub parent: Option<String>,
    pub names: Option<Vec<CategoryProductNameAggregation>>,
    pub short_descriptions: Option<Vec<CategoryProductDescriptionAggregation>>,
    pub long_descriptions: Option<Vec<CategoryProductDescriptionAggregation>>,
    pub price: Option<CategoryProductPriceAggregation>,
    pub discount: Option<CategoryProductDiscountAggregation>,
    pub unit: Option<CategoryProductUnitAggregation>,
    pub picture: Option<CategoryProductPictureAggregation>,
    pub copied_pictures: Option<Vec<CategoryProductPictureAggregation>>,
    pub order: Option<i32>,
    pub is_published: Option<bool>,
    pub is_available: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub choices: Option<Vec<CategoryProductChoiceAggregation>>,
    pub prep_time: Option<i32>
    //pub availabilities: Option<Vec<CategoryProductUnavailabilityAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductNameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductDescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductPriceAggregation {
    pub value: Option<f64>,
    pub currency: Option<CategoryProductPriceCurrencyAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductPriceCurrencyAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductDiscountAggregation {
    pub percentage: Option<i32>,
    pub price: Option<CategoryProductPriceAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductUnitAggregation {
    pub kind: Option<String>,
    pub default: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductPictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductChoiceAggregation {
    pub id: Option<String>,
    pub names: Option<Vec<CategoryProductChoiceNameAggregation>>,
    pub descriptions: Option<Vec<CategoryProductChoiceDescriptionAggregation>>,
    pub min_items: Option<i32>,
    pub max_items: Option<i32>,
    pub supplements: Option<Vec<ChoiceSupplementAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductChoiceNameAggregation {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductChoiceDescriptionAggregation {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceSupplementAggregation {
    pub id: Option<String>,
    pub names: Option<Vec<CategoryProductChoiceNameAggregation>>,
    pub descriptions: Option<Vec<CategoryProductChoiceDescriptionAggregation>>,
    pub price: Option<CategoryProductChoicePriceAggregation>,
    pub discount: Option<CategoryProductChoiceDiscountAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductChoicePriceAggregation {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductChoiceDiscountAggregation {
    pub percentage: Option<i32>,
    pub price: Option<CategoryProductChoicePriceAggregation>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct CategoryProductUnavailabilityAggregation {
//     pub site: Option<String>,
// }

#[derive(Debug, Display)]
pub enum GetProductDetailsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductDetailsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductDetailsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductDetailsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
