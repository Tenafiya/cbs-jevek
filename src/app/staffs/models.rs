use chrono::{DateTime, FixedOffset, NaiveDate};
use entity::sea_orm_active_enums::{StaffEmploymentEnum, StaffGenderEnum};
use sea_orm::{FromQueryResult, entity::prelude::*, prelude::Decimal};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{DisplayFromStr, serde_as};
use validator::Validate;

use crate::utils::validators::validate_birth_date;

#[derive(Debug, Clone)]
pub struct SetupStaff {
    pub institution_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
    pub salt: uuid::Uuid,
}

#[derive(Debug, Clone)]
pub struct AddStaffModel {
    pub institution_id: i64,
    pub branch_id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: String,
    pub gender: StaffGenderEnum,
    pub nationality: String,
    pub job_title: String,
    pub hired_date: NaiveDate,
    pub password: String,
    pub salt: uuid::Uuid,
}

#[derive(Debug, Clone)]
pub struct UpdateStaffStatusModel {
    pub id: i64,
    pub employment_status: StaffEmploymentEnum,
}

#[derive(Debug, Clone)]
pub struct UpdateStaffModel {
    pub branch_id: Option<i64>,
    pub phone_number: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub department: Option<String>,
    pub job_title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GetAuthModel {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddInitializerParams {
    #[serde(rename = "institutionName")]
    pub institution_name: String,
    #[validate(length(min = 2, max = 50, message = "first name is invalid"))]
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[validate(length(min = 2, max = 50, message = "last name is invalid"))]
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[validate(length(equal = 10, message = "phone number is invalid"))]
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddStaffParams {
    #[serde(rename = "institutionId")]
    pub institution_id: String,
    #[serde(rename = "branchId")]
    pub branch_id: String,
    #[validate(length(min = 2, max = 50, message = "first name is invalid"))]
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[validate(length(min = 2, max = 50, message = "last name is invalid"))]
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[validate(length(equal = 10, message = "phone number is invalid"))]
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[validate(email)]
    pub email: String,
    pub gender: StaffGenderEnum,
    #[validate(length(min = 2, max = 50, message = "nationality is invalid"))]
    pub nationality: String,
    #[validate(length(min = 2, max = 50, message = "job title is invalid"))]
    #[serde(rename = "jobTitle")]
    pub job_title: String,
    #[serde(rename = "hiredDate")]
    pub hired_date: NaiveDate,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateStaffParams {
    #[serde(rename = "branchId")]
    pub branch_id: Option<String>,
    #[validate(length(equal = 10, message = "phone number is invalid"))]
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
    #[serde(rename = "dateOfBirth")]
    #[validate(custom(function = "validate_birth_date"))]
    pub birth_date: Option<NaiveDate>,
    #[validate(length(min = 2, max = 50, message = "department is invalid"))]
    #[serde(rename = "department")]
    pub department: Option<String>,
    #[validate(length(min = 2, max = 50, message = "job title is invalid"))]
    #[serde(rename = "jobTitle")]
    pub job_title: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct SignInParams {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateStaffStatusParams {
    #[serde(rename = "staffId")]
    pub staff_id: String,
    pub status: StaffEmploymentEnum,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "entity::staff::Entity")]
pub struct StaffResponseModel {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "_id")]
    #[sea_orm(from_col = "id")]
    pub id: i64,
    #[serde_as(as = "DisplayFromStr")]
    #[sea_orm(from_col = "institution_id")]
    pub institution_id: i64,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[sea_orm(from_col = "branch_id")]
    pub branch_id: Option<i64>,
    #[sea_orm(from_col = "employee_number")]
    pub employee_number: String,
    #[sea_orm(from_col = "first_name")]
    pub first_name: String,
    #[sea_orm(from_col = "last_name")]
    pub last_name: String,
    #[sea_orm(from_col = "full_name")]
    pub full_name: Option<String>,
    #[sea_orm(from_col = "phone_number")]
    pub phone_number: String,
    #[serde(rename = "email")]
    #[sea_orm(from_col = "email_address")]
    pub email_address: String,
    #[sea_orm(from_col = "date_of_birth")]
    pub date_of_birth: Option<NaiveDate>,
    #[sea_orm(from_col = "gender")]
    pub gender: Option<StaffGenderEnum>,
    #[serde(rename = "occupation")]
    #[sea_orm(from_col = "job_title")]
    pub job_title: Option<String>,
    #[sea_orm(from_col = "nationality")]
    pub nationality: Option<String>,
    #[sea_orm(from_col = "department")]
    pub department: Option<String>,
    #[sea_orm(from_col = "employment_status")]
    pub employment_status: Option<StaffEmploymentEnum>,
    #[sea_orm(from_col = "date_hired")]
    pub date_hired: Option<NaiveDate>,
    #[sea_orm(from_col = "date_terminated")]
    pub date_terminated: Option<NaiveDate>,
    #[sea_orm(from_col = "termination_reason")]
    pub termination_reason: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[sea_orm(from_col = "role_id")]
    pub role_id: Option<i64>,
    #[sea_orm(from_col = "permissions")]
    pub permissions: Option<Value>,
    #[sea_orm(from_col = "is_password_changed")]
    pub is_password_changed: Option<bool>,
    #[sea_orm(from_col = "password_last_changed_at")]
    pub password_last_changed_at: Option<DateTime<FixedOffset>>,
    #[sea_orm(from_col = "is_mfa_enabled")]
    pub is_mfa_enabled: Option<bool>,
    #[sea_orm(from_col = "session")]
    pub session: Option<uuid::Uuid>,
    #[sea_orm(from_col = "performance_rating")]
    pub performance_rating: Option<Decimal>,
    #[sea_orm(from_col = "last_appraisal_date")]
    pub last_appraisal_date: Option<NaiveDate>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[sea_orm(from_col = "supervisor_id")]
    pub supervisor_id: Option<i64>,
    #[sea_orm(from_col = "custom_fields")]
    pub custom_fields: Option<Value>,
    #[sea_orm(from_col = "created_at")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[sea_orm(from_col = "updated_at")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}
