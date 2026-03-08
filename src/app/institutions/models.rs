use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;
use serde_with::{serde_as, DisplayFromStr};
use sea_orm::{FromQueryResult, entity::prelude::*};
use chrono::{FixedOffset, DateTime};

#[serde_as]
#[derive(Debug, Clone, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "entity::institutions::Entity")]
pub struct InstitutionResponseModel {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "_id")]
    #[sea_orm(from_col = "id")]
    pub id: i64,
    #[sea_orm(from_col = "name")]
    pub name: String,
    #[sea_orm(from_col = "code")]
    pub code: Option<String>,
    #[sea_orm(from_col = "timezone")]
    pub timezone: Option<String>,
    #[sea_orm(from_col = "license_number")]
    pub license_number: Option<String>,
    #[sea_orm(from_col = "regulatory_number")]
    pub regulatory_number: Option<String>,
    #[sea_orm(from_col = "logo_url")]
    pub logo_url: Option<String>,
    #[sea_orm(from_col = "city")]
    pub city: Option<String>,
    #[sea_orm(from_col = "zip_code")]
    pub zip_code: Option<String>,
    #[sea_orm(from_col = "state")]
    pub state: Option<String>,
    #[sea_orm(from_col = "date_format")]
    pub date_format: Option<String>,
    #[sea_orm(from_col = "date_time_format")]
    pub date_time_format: Option<String>,
    #[sea_orm(from_col = "address")]
    pub address: Option<Value>,
    #[sea_orm(from_col = "postal_address")]
    pub postal_address: Option<Value>,
    #[sea_orm(from_col = "is_active")]
    pub is_active: Option<bool>,
    #[sea_orm(from_col = "is_deleted")]
    pub is_deleted: Option<bool>,
    #[sea_orm(from_col = "deleted_at")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[sea_orm(from_col = "created_at")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[sea_orm(from_col = "updated_at")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone)]
pub struct AddInstitutionModel {
    pub name: String,
    pub code: String,
    pub country: i64,
    pub license_num: String,
    pub regulation_num: String,
    pub city: String,
    pub zip_code: String,
    pub state: String,
    pub date_format: String,
    pub date_time_format: String,
    pub address: Value,
    pub postal_address: Value,
}

#[derive(Debug, Clone)]
pub struct UpdateInstitutionModel {
    pub name: Option<String>,
    pub timezone: Option<String>,
    pub license_num: Option<String>,
    pub regulation_num: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddInstitutionParams {
    #[validate(length(min = 3, max = 120, message = "Name is invalid"))]
    pub name: String,
    #[serde(rename = "countryId")]
    pub country_id: String,
    #[serde(rename = "licenseNumber")]
    pub license_num: String,
    #[serde(rename = "regulatoryNumber")]
    pub regulation_num: String,
    #[serde(rename = "gpsCode")]
    pub zip_code: String,
    pub city: String,
    #[serde(rename = "region")]
    pub state: String,
    #[serde(rename = "dateFormat")]
    pub date_format: String,
    #[serde(rename = "dateTimeFormat")]
    pub date_time_format: String,
    pub address: Value,
    #[serde(rename = "postalAddress")]
    pub postal_address: Value
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateInstitutionParams {
    pub id: String,
    #[validate(length(min = 3, max = 120, message = "Name is invalid"))]
    pub name: Option<String>,
    #[serde(rename = "licenseNumber")]
    pub license_num: Option<String>,
    #[serde(rename = "regulatoryNumber")]
    pub regulation_num: Option<String>,
    pub timezone: Option<String>,
}
