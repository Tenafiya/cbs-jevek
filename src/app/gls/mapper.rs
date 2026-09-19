use chrono::{DateTime, FixedOffset, NaiveDate};
use entity::sea_orm_active_enums::LedgerLockType;
use sea_orm::DerivePartialModel;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Debug, Clone, Serialize, DerivePartialModel)]
#[sea_orm(entity = "entity::ledger_lock_periods::Entity")]
pub struct LedgerLockModel {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "_id")]
    #[sea_orm(from_col = "id")]
    pub id: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[sea_orm(from_col = "institution_id")]
    pub institution_id: i64,

    #[sea_orm(from_col = "start_date")]
    pub start_date: NaiveDate,

    #[sea_orm(from_col = "end_date")]
    pub end_date: NaiveDate,

    #[sea_orm(from_col = "lock_type")]
    pub lock_type: LedgerLockType,

    #[sea_orm(from_col = "is_locked")]
    pub is_locked: Option<bool>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[sea_orm(from_col = "locked_by")]
    pub locked_by: Option<i64>,

    #[sea_orm(from_col = "locked_at")]
    pub locked_at: Option<DateTime<FixedOffset>>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[sea_orm(from_col = "locked_by")]
    pub unlocked_by: Option<i64>,

    #[sea_orm(from_col = "unlocked_at")]
    pub unlocked_at: Option<DateTime<FixedOffset>>,

    #[sea_orm(from_col = "unlock_reason")]
    pub unlock_reason: Option<String>,

    #[sea_orm(from_col = "created_at")]
    pub created_at: Option<DateTime<FixedOffset>>,

    #[sea_orm(from_col = "updated_at")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}
