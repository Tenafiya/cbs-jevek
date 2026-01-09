use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

use crate::utils::validators::validate_operation;

// DTO MODELS
#[derive(Debug, Clone)]
pub struct AddCountryModel {
    pub name: String,
    pub official_name: String,
    pub capital_city: String,
    pub call_code: String,
    pub iso_code: String,
    pub flag_url: String,
    pub currency: Value,
    pub more_data: Option<Value>,
}

// Impl From
#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct CountryResponseModel {
    pub id: String,
    pub name: String,
    pub official_name: Option<String>,
    pub capital_city: Option<String>,
    pub currency: Option<Value>,
    pub flag_url: Option<String>,
    pub call_code: String,
    pub iso_code: String,
}

impl From<entity::countries::Model> for CountryResponseModel {
    fn from(country: entity::countries::Model) -> Self {
        Self {
            id: country.slug,
            name: country.name,
            official_name: country.official_name,
            capital_city: country.capital_city,
            currency: country.currency,
            flag_url: country.flag_url,
            call_code: country.call_code,
            iso_code: country.iso_code,
        }
    }
}

//Params
#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddCountryParamsModel {
    #[validate(length(min = 3, max = 120, message = "Name is invalid"))]
    pub name: String,
    #[validate(length(min = 3, max = 120, message = "Official Name is invalid"))]
    #[serde(rename = "officialName")]
    pub official_name: String,
    #[validate(length(min = 3, max = 120, message = "Capital City is invalid"))]
    #[serde(rename = "capitalCity")]
    pub capital_city: String,
    #[validate(length(min = 3, max = 120, message = "Call Code is invalid"))]
    #[serde(rename = "callCode")]
    pub call_code: String,
    #[validate(length(min = 1, max = 120, message = "Iso Code is invalid"))]
    #[serde(rename = "isoCode")]
    pub iso_code: String,
    #[validate(length(min = 3, max = 120, message = "Flag Url is invalid"))]
    #[serde(rename = "flagUrl")]
    pub flag_url: String,
    pub currency: Value,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct OperationModel {
    pub id: String,
    #[validate(custom(function = "validate_operation"))]
    pub operation: String,
}
