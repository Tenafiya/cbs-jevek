use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCategorySummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: Option<String>,
    pub category_type: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct QueryModel {
    pub size: u64,
    pub page: u64,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathParamsModel {
    pub id: String,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryParamsModel {
    #[serde(default = "default_per_page")]
    pub size: u64,
    #[serde(default = "default_page")]
    pub page: u64,
}

#[derive(Debug, Serialize)]
pub struct MetaModel {
    pub total_items: u64,
    pub total_pages: u64,
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Serialize)]
pub struct ListResponseModel<T> {
    pub items: T,
    pub meta: MetaModel,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    10
}

pub fn default_decimal() -> Decimal {
    Decimal::ZERO
}
