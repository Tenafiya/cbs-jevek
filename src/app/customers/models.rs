use chrono::NaiveDate;
use entity::sea_orm_active_enums::CustomerType;
use sea_orm::prelude::Decimal;
use serde::Deserialize;
use serde_json::Value;
use validator::Validate;

use crate::utils::{
    models::default_decimal,
    validators::{validate_date, validate_gender, validate_income},
};

#[derive(Debug, Clone)]
pub struct AddCustomerModel {
    pub institution_id: i64,
    pub customer_type: CustomerType,
    pub customer_num: String,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub birth_date: NaiveDate,
    pub gender: String,
    pub nationality: String,
    pub phone_number: String,
    pub phone_country_code: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AddAddressModel {
    pub residential_address: Option<Value>,
    pub postal_address: Option<Value>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct AddOccupationModel {
    pub occupation: String,
    pub employer_name: Option<String>,
    pub income_source: Option<String>,
    pub monthly_income: Decimal,
}

#[derive(Debug, Clone)]
pub struct AddNextModel {
    pub next_of_kin: Option<Value>,
    pub pep_details: Option<Value>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddCustomerParams {
    #[serde(rename = "customerType")]
    pub customer_type: CustomerType,
    #[validate(length(min = 1, max = 50, message = "first name cannot be < 1 and > 50"))]
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[validate(length(min = 1, max = 50, message = "last name cannot be < 1 and > 50"))]
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[validate(length(min = 1, max = 100, message = "Middle name cannot be < 1 and > 50"))]
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
    #[serde(rename = "dateOfBirth")]
    #[validate(custom(function = "validate_date"))]
    pub birth_date: NaiveDate,
    #[validate(custom(function = "validate_gender"))]
    pub gender: String,
    #[validate(length(min = 2, max = 150, message = "Nationality cannot be < 2 and > 150"))]
    pub nationality: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[serde(rename = "phoneCountryCode")]
    pub phone_country_code: String,
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddAddressParams {
    #[serde(rename = "residentialAddress")]
    pub residential_address: Value,
    #[serde(rename = "postalAddress")]
    pub postal_address: Value,
    #[validate(length(min = 1, max = 50, message = "City cannot be < 1 and > 50"))]
    pub city: String,
    #[validate(length(min = 1, max = 50, message = "State cannot be < 1 and > 50"))]
    pub state: String,
    #[serde(rename = "countryId")]
    pub country_id: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddOccupationParams {
    #[validate(length(min = 5, max = 50, message = "Occupation cannot be < 5 and > 50"))]
    pub occupation: String,
    #[validate(length(min = 2, max = 100, message = "Employer Name cannot be < 2 and > 100"))]
    #[serde(rename = "employerName")]
    pub employer_name: Option<String>,
    #[validate(length(min = 2, max = 50, message = "Income source cannot be < 2 and > 50"))]
    #[serde(rename = "sourceOfIncome")]
    pub income_source: Option<String>,
    #[serde(default = "default_decimal")]
    #[validate(custom(function = "validate_income"))]
    #[serde(rename = "monthlyIncome")]
    pub monthly_income: Decimal,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct NextOfKinParams {
    #[serde(rename = "nextOfKin")]
    pub next_of_kin: Option<Value>,
    #[serde(rename = "pepDetails")]
    pub pep_details: Option<Value>,
}
