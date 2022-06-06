use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateCategoryProductBody {
    pub id: String,
    pub category: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<UpdateCategoryProductNameBody>>,
    pub short_descriptions: Option<Vec<UpdateCategoryProductDescriptionBody>>,
    pub long_descriptions: Option<Vec<UpdateCategoryProductDescriptionBody>>,
    pub price: Option<CategoryProductPriceBody>,
    pub discount: Option<CategoryProductDiscountBody>,
    pub unit: Option<CategoryProductUnitBody>,
    pub picture: Option<String>,
    pub copied_pictures: Option<Vec<CategoryProductCopiedPictureBody>>,
    pub order: Option<i32>,
    pub is_published: Option<bool>,
    pub is_available: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub choices: Option<Vec<CategoryProductChoiceBody>>,
    pub prep_time: Option<i32>
    // pub unavailability: Option<Vec<CategoryProductUnavailabilityBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCategoryProductNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCategoryProductDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductPriceBody {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductDiscountBody {
    pub percentage: Option<i32>,
    pub price: Option<CategoryProductPriceBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductUnitBody {
    pub kind: Option<String>,
    pub default: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductCopiedPictureBody {
    pub id: Option<String>,
    pub quality: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryProductChoiceBody {
    pub id: Option<String>,
    pub names: Option<Vec<UpdateChoiceNameBody>>,
    pub descriptions: Option<Vec<UpdateChoiceDescriptionBody>>,
    pub min_items: Option<i32>,
    pub max_items: Option<i32>,
    pub supplements: Option<Vec<ChoiceSupplementBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateChoiceNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateChoiceDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceSupplementBody {
    pub id: Option<String>,
    pub names: Option<Vec<UpdateChoiceSupplementNameBody>>,
    pub descriptions: Option<Vec<UpdateChoiceSupplementDescriptionBody>>,
    pub price: Option<ChoiceSupplementPriceBody>,
    pub discount: Option<ChoiceSupplementDiscountBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateChoiceSupplementNameBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateChoiceSupplementDescriptionBody {
    pub id: Option<String>,
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceSupplementPriceBody {
    pub value: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceSupplementDiscountBody {
    pub percentage: Option<i32>,
    pub price: Option<CategoryProductPriceBody>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct CategoryProductUnavailabilityBody {
//     pub site: Option<String>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCategoryProductResult {
    pub success: bool,
}

#[derive(Debug, Display)]
pub enum UpdateCategoryProductError {
    #[display(fmt = "invalid_object_id")]
    InvalidObjectId,
    #[display(fmt = "category_product_not_found")]
    CategoryProductNotFound,
    #[display(fmt = "identifier_is_not_alphabectic")]
    IdentifierIsNotAplhabetic,
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    Default(String),
}

impl ResponseError for UpdateCategoryProductError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UpdateCategoryProductError::InvalidObjectId => {
                HttpResponse::NotAcceptable().body("invalid_object_id")
            }
            UpdateCategoryProductError::CategoryProductNotFound => {
                HttpResponse::Conflict().body("category_product_not_found")
            }
            UpdateCategoryProductError::IdentifierIsNotAplhabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabectic")
            }
            UpdateCategoryProductError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            UpdateCategoryProductError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
