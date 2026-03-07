use sea_orm::{FromQueryResult, entity::prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr};
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
    pub cash_limit: i64,
    #[serde(rename = "isMainBranch")]
    pub is_main_branch: bool,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "entity::branches::Entity")]
pub struct BranchResponseModel {
    #[serde_as(as = "DisplayFromStr")]
    #[sea_orm(from_col = "id")]
    pub id: i64,
    #[serde_as(as = "DisplayFromStr")]
    #[sea_orm(from_col = "institution_id")]
    pub institution_id: i64,
}
