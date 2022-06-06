use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateCategoryProductBody {
    pub category: Option<String>,
    pub identifier: Option<String>,
    pub names: Option<Vec<CreateCategoryProductNameBody>>,
    pub short_descriptions: Option<Vec<CreateCategoryProductDescriptionBody>>,
    pub long_descriptions: Option<Vec<CreateCategoryProductDescriptionBody>>,
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
    //pub unavailability: Option<Vec<CategoryProductUnavailabilityBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCategoryProductNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateCategoryProductDescriptionBody {
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
    pub names: Option<Vec<CreateChoiceNameBody>>,
    pub descriptions: Option<Vec<CreateChoiceDescriptionBody>>,
    pub min_items: Option<i32>,
    pub max_items: Option<i32>,
    pub supplements: Option<Vec<ChoiceSupplementBody>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChoiceNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChoiceDescriptionBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceSupplementBody {
    pub names: Option<Vec<CreateChoiceSupplementNameBody>>,
    pub descriptions: Option<Vec<CreateChoiceSupplementDescriptionBody>>,
    pub price: Option<ChoiceSupplementPriceBody>,
    pub discount: Option<ChoiceSupplementDiscountBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChoiceSupplementNameBody {
    pub language_code: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateChoiceSupplementDescriptionBody {
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
pub struct CreateCategoryProductResult {
    pub id: String,
}

#[derive(Debug, Display)]
pub enum CreateCategoryProductError {
    #[display(fmt = "identifier_already_exists")]
    IdentifierExists,
    #[display(fmt = "identifier_is_not_alphabectic")]
    IdentifierIsNotAplhabetic,
    Default(String),
}

impl ResponseError for CreateCategoryProductError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CreateCategoryProductError::IdentifierExists => {
                HttpResponse::Conflict().body("identifier_already_exists")
            }
            CreateCategoryProductError::IdentifierIsNotAplhabetic => {
                HttpResponse::NotAcceptable().body("identifier_is_not_alphabectic")
            }
            CreateCategoryProductError::Default(error) => HttpResponse::BadRequest().body(error),
        }
    }
}
