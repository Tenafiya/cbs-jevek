use sea_orm::{FromQueryResult, entity::prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr};
use validator::Validate;

use crate::utils::validators::validate_operation;

#[serde_as]
#[derive(Debug, Clone, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "entity::countries::Entity")]
pub struct CountryResponseModel {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "_id")] 
    #[sea_orm(from_col = "id")]
    pub id: i64,
    #[sea_orm(from_col = "slug")]
    pub slug: String,
    #[sea_orm(from_col = "name")]
    pub name: String,
    #[sea_orm(from_col = "official_name")]
    pub official_name: String,
    #[sea_orm(from_col = "capital_city")]
    pub capital_city: String,
    #[sea_orm(from_col = "call_code")]
    pub call_code: String,
    #[sea_orm(from_col = "iso_code")]
    pub iso_code: String,
    #[sea_orm(from_col = "flag_url")]
    pub flag_url: String,
    #[sea_orm(from_col = "currency")]
    pub currency: Value,
    #[sea_orm(from_col = "more_data")]
    pub more_data: Option<Value>,
}

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
