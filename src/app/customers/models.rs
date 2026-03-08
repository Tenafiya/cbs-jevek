use chrono::{FixedOffset, DateTime, NaiveDate};
use entity::sea_orm_active_enums::CustomerType;
use sea_orm::{FromQueryResult, entity::prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr};
use validator::Validate;

use crate::utils::{
    models::default_decimal,
    validators::{validate_birth_date, validate_gender, validate_income},
};

#[serde_as]
#[derive(Debug, Clone, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "entity::customers::Entity")]
pub struct CustomerResponseModel {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "_id")]
    #[sea_orm(from_col = "id")]
    pub id: i64,
    #[serde_as(as = "DisplayFromStr")]
    #[sea_orm(from_col = "institution_id")]
    pub institution_id: i64,
    #[sea_orm(from_col = "customer_type")]
    pub customer_type: Option<CustomerType>,
    #[sea_orm(from_col = "customer_number")]
    pub customer_number: Option<String>,
     #[sea_orm(from_col = "risk_level")]
    pub risk_level: Option<String>,
     #[sea_orm(from_col = "status")]
    pub status: Option<String>,
     #[sea_orm(from_col = "is_black_listed")]
    pub is_black_listed: Option<bool>,
    #[sea_orm(from_col = "black_list_reason")]
    pub black_list_reason: Option<String>,
    #[sea_orm(from_col = "first_name")]
    pub first_name: Option<String>,
    #[sea_orm(from_col = "last_name")]
    pub last_name: Option<String>,
    #[sea_orm(from_col = "middle_name")]
    pub middle_name: Option<String>,
    #[sea_orm(from_col = "date_of_birth")]
    pub date_of_birth: Option<NaiveDate>,
    #[sea_orm(from_col = "gender")]
    pub gender: Option<String>,
    #[sea_orm(from_col = "nationality")]
    pub nationality: Option<String>,
    #[sea_orm(from_col = "phone_number")]
    pub phone_number: Option<String>,
    #[sea_orm(from_col = "email")]
    pub email: Option<String>,
    #[sea_orm(from_col = "is_phone_verified")]
    pub is_phone_verified: Option<String>,
    #[sea_orm(from_col = "is_email_verified")]
    pub is_email_verified: Option<String>,
    #[sea_orm(from_col = "phone_verified_at")]
    pub phone_verified_at: Option<DateTime<FixedOffset>>,
    #[sea_orm(from_col = "email_verified_at")]
    pub email_verified_at: Option<DateTime<FixedOffset>>,
    #[sea_orm(from_col = "residential_address")]
    pub residential_address: Option<Value>,
    #[sea_orm(from_col = "postal_address")]
    pub postal_address: Option<Value>,
    #[sea_orm(from_col = "city")]
    pub city: Option<String>,
    #[sea_orm(from_col = "state_province")]
    pub state_province: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[sea_orm(from_col = "country_id")]
    pub country_id: Option<i64>,
    #[sea_orm(from_col = "occupation")]
    pub occupation: Option<String>,
    #[sea_orm(from_col = "employer_name")]
    pub employer_name: Option<String>,
    #[sea_orm(from_col = "income_source")]
    pub income_source: Option<String>,
    #[sea_orm(from_col = "monthly_income")]
    pub monthly_income: Option<Decimal>,
    #[sea_orm(from_col = "employee_number")]
    pub employee_number: Option<String>,
    #[sea_orm(from_col = "next_of_kin")]
    pub next_of_kin: Option<Value>,
    #[sea_orm(from_col = "is_pep")]
    pub is_pep: Option<bool>,
    #[sea_orm(from_col = "pep_details")]
    pub pep_details: Option<Value>,
    #[sea_orm(from_col = "is_sanctions_check_passed")]
    pub is_sanctions_check_passed: Option<bool>,
    #[sea_orm(from_col = "sanctions_check_date")]
    pub sanctions_check_date: Option<String>,
    #[sea_orm(from_col = "sanctions_provider")]
    pub sanctions_provider: Option<String>,
    #[sea_orm(from_col = "custom_fields")]
    pub custom_fields: Option<Value>,
    #[sea_orm(from_col = "tags")]
    pub tags: Option<Vec<String>>,
    #[sea_orm(from_col = "verified_at")]
    pub verified_at: Option<DateTime<FixedOffset>>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[sea_orm(from_col = "created_by")]
    pub created_by: Option<i64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[sea_orm(from_col = "verified_by")]
    pub verified_by: Option<i64>,
    #[sea_orm(from_col = "kyc_tier")]
    pub kyc_tier: Option<Value>,
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
pub struct AddCustomerModel {
    pub institution_id: i64,
    pub customer_type: CustomerType,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub birth_date: NaiveDate,
    pub gender: String,
    pub nationality: String,
    pub phone_number: String,
    pub phone_country_code: String,
    pub email: Option<String>,
    pub created_by: i64,
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
    #[validate(custom(function = "validate_birth_date"))]
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
