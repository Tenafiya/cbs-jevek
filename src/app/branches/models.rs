use crate::utils::conversions;
use chrono::{DateTime, FixedOffset};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{DisplayFromStr, serde_as};
use validator::Validate;

#[derive(Debug, Clone)]
pub struct AddBranchModel {
    pub name: String,
    pub code: String,
    pub institution: i64,
    pub address: Value,
    pub phone: String,
    pub email: String,
    pub location: Value,
    pub is_main: bool,
    pub cash_limit: i64,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddBranchParams {
    #[validate(length(min = 3, max = 120, message = "Name is invalid"))]
    pub name: String,

    #[serde(rename = "institutionId")]
    pub institution_id: String,

    pub address: Value,

    pub phone: String,

    pub email: String,

    pub location: Value,

    #[serde(rename = "cashLimit")]
    pub cash_limit: Decimal,

    #[serde(rename = "isMainBranch")]
    pub is_main_branch: bool,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, DerivePartialModel)]
#[sea_orm(entity = "entity::branches::Entity")]
pub struct BranchResponseModel {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "_id")]
    #[sea_orm(from_col = "id")]
    pub id: i64,
    #[serde_as(as = "DisplayFromStr")]
    #[sea_orm(from_col = "institution_id")]
    pub institution_id: i64,
    #[sea_orm(from_col = "name")]
    pub name: Option<String>,
    #[sea_orm(from_col = "code")]
    pub code: Option<String>,
    #[sea_orm(from_col = "address")]
    pub address: Option<Value>,
    #[sea_orm(from_col = "phone")]
    pub phone: Option<String>,
    #[sea_orm(from_col = "email")]
    pub email: Option<String>,
    #[sea_orm(from_col = "location")]
    pub location: Option<Value>,
    #[serde(rename = "default_branch")]
    #[sea_orm(from_col = "is_main")]
    pub is_main: Option<bool>,
    #[sea_orm(from_col = "cash_limit")]
    pub cash_limit: Option<i64>,
    #[sea_orm(from_col = "is_active")]
    pub is_active: Option<bool>,
    #[sea_orm(from_col = "status")]
    pub status: Option<String>,
    #[sea_orm(from_col = "is_deleted")]
    pub is_deleted: Option<bool>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[sea_orm(from_col = "manager_id")]
    pub manager_id: Option<i64>,
    #[sea_orm(from_col = "deleted_at")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[sea_orm(from_col = "created_at")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[sea_orm(from_col = "updated_at")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BranchResponse {
    #[serde(rename = "_id")]
    pub id: i64,

    pub institution_id: i64,

    pub name: Option<String>,

    pub code: Option<String>,

    pub address: Option<Value>,

    pub phone: Option<String>,

    pub email: Option<String>,

    pub location: Option<Value>,

    #[serde(rename = "default_branch")]
    pub is_main: Option<bool>,

    pub cash_limit: Option<Decimal>,

    pub is_active: Option<bool>,

    pub status: Option<String>,

    pub is_deleted: Option<bool>,

    pub manager_id: Option<i64>,

    pub deleted_at: Option<DateTime<FixedOffset>>,

    pub created_at: Option<DateTime<FixedOffset>>,

    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl BranchResponseModel {
    pub fn into_response(self) -> BranchResponse {
        BranchResponse {
            id: self.id,
            institution_id: self.institution_id,
            name: self.name,
            code: self.code,
            address: self.address,
            phone: self.phone,
            email: self.email,
            location: self.location,
            is_main: self.is_main,
            cash_limit: self
                .cash_limit
                .map(|amount| conversions::major_conversion(amount, "GHS")),
            is_active: self.is_active,
            status: self.status,
            is_deleted: self.is_deleted,
            manager_id: self.manager_id,
            deleted_at: self.deleted_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
