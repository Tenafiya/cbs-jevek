use serde::Deserialize;
use serde_json::Value;
use validator::Validate;

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
