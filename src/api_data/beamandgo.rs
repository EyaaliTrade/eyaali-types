use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiData {
    pub beamandgo: Option<Beamandgo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Beamandgo {
    pub productSid: Option<String>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub productTypeId: Option<i32>,
    pub productTypeName: Option<String>,
    pub unitPrice: Option<i32>,
    pub unitPriceCurrencyCode: Option<String>,
    pub imageUrl: Option<String>,
    pub minQty: Option<i32>,
    pub maxQty: Option<i32>,
    pub stockQty: Option<i32>,
    pub defaultQty: Option<i32>,
    pub isBranchSpecific: Option<bool>,
    pub isShippingRequired: Option<bool>,
    pub isBirthDateRequired: Option<bool>,
    pub accountSid: Option<String>,
    pub shippingAndHandlingFee: Option<i32>
}