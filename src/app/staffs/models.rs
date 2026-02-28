use chrono::NaiveDate;
use entity::sea_orm_active_enums::{StaffEmploymentEnum, StaffGenderEnum};
use serde::Deserialize;
use validator::Validate;

use crate::utils::validators::validate_birth_date;

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
    pub data_of_birth: Option<NaiveDate>,
    pub department: Option<String>,
    pub job_title: Option<String>,
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
pub struct UpdateStaffStatusParams {
    #[serde(rename = "staffId")]
    pub staff_id: String,
    pub status: StaffEmploymentEnum,
}
