use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductsBody {
    pub menus: Vec<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductsResult {
    pub list: Vec<ProductAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub category: Option<CategoryProductAggregation>,
    pub names: Option<Vec<NameAggregation>>,
    pub short_descriptions: Option<Vec<DescriptionAggregation>>,
    pub long_descriptions: Option<Vec<DescriptionAggregation>>,
    pub price: Option<ProductPriceAggregation>,
    pub discount: Option<ProductDiscountAggregation>,
    pub unit: Option<ProductUnitAggregation>,
    pub picture: Option<ProductPictureAggregation>,
    pub copied_pictures: Option<Vec<ProductPictureAggregation>>,
    pub order: Option<i32>,
    pub is_published: Option<bool>,
    pub is_available: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub choices: Option<Vec<CategoryProductChoiceAggregation>>,
    //pub availabilities: Option<Vec<CategoryProductUnavailabilityAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DescriptionAggregation {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPriceAggregation {
    pub value: Option<f64>,
    pub currency: Option<ProductPriceCurrencyAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPriceCurrencyAggregation {
    pub id: Option<String>,
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductDiscountAggregation {
    pub percentage: Option<i32>,
    pub price: Option<ProductPriceAggregation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductUnitAggregation {
    pub kind: Option<String>,
    pub default: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductPictureAggregation {
    pub id: Option<String>,
    pub quality: Option<String>,
    pub kind: Option<String>,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<NameAggregation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubCategoryProductAggregation {
    pub id: Option<String>,
    pub identifier: Option<String>,
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
pub enum GetProductsError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    Default(String),
}

impl ResponseError for GetProductsError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GetProductsError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            GetProductsError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
