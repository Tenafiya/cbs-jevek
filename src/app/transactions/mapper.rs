use chrono::NaiveDate;
use chrono::{DateTime, FixedOffset};
use entity::sea_orm_active_enums::{
    CustomerType, StaffEmploymentEnum, TellerCashDrawersStatus, TransactionCategoryType,
    TransactionLimitsLimitType, TransactionStatus, TransactionType,
};
use rust_decimal::Decimal;
use sea_orm::DerivePartialModel;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{DisplayFromStr, serde_as};

use crate::utils::conversions;
use crate::utils::models::{AccountCategorySummary, TellerCashDrawerSummary, TellerSummary};
use crate::utils::models::{StaffSummary, TransactionSummary};

// ===========================================
// Transaction Checker
// ===========================================
#[serde_as]
#[derive(Debug, Clone, Serialize, DerivePartialModel)]
#[sea_orm(entity = "entity::transaction_channels::Entity")]
pub struct TransactionChannelResponseModel {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "_id")]
    #[sea_orm(from_col = "id")]
    pub id: i64,
    #[serde_as(as = "DisplayFromStr")]
    #[sea_orm(from_col = "institution_id")]
    pub institution_id: i64,
    #[sea_orm(from_col = "channel_name")]
    pub channel_name: Option<String>,
    #[sea_orm(from_col = "channel_code")]
    pub channel_code: Option<String>,
    #[sea_orm(from_col = "description")]
    pub description: Option<String>,
    #[sea_orm(from_col = "is_active")]
    pub is_active: Option<bool>,
    #[sea_orm(from_col = "requires_maker_checker")]
    pub requires_maker_checker: bool,
    #[sea_orm(from_col = "metadata")]
    pub metadata: Option<Value>,
    #[sea_orm(from_col = "created_at")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[sea_orm(from_col = "updated_at")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

// ===========================================
// Transaction Checker
// ===========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionChannelSummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub channel_name: Option<String>,
    pub channel_code: Option<String>,
    pub requires_maker_checker: Option<bool>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionLimitSummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub transaction_channel_id: String,
    pub account_category_id: String,
    pub customer_type: CustomerType,
    pub limit_type: TransactionLimitsLimitType,
    pub max_amount: Option<i64>,
    pub max_count: Option<i64>,
    pub currency: Option<Value>,
    pub effective_from: Option<DateTime<FixedOffset>>,
    pub effective_to: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionCheckerRow {
    pub channel: TransactionChannelSummary,
    pub limit: Option<TransactionLimitSummary>,
}

#[derive(FromQueryResult, Debug, Clone)]
pub struct TransactionCheckerFlat {
    pub id: i64,
    pub institution_id: i64,
    pub channel_name: Option<String>,
    pub channel_code: Option<String>,
    pub requires_maker_checker: Option<bool>,
    pub metadata: Option<Value>,

    pub limit_id: Option<i64>,
    pub limit_institution_id: i64,
    pub limit_transaction_channel_id: i64,
    pub limit_account_category_id: i64,
    pub limit_customer_type: CustomerType,
    pub limit_limit_type: TransactionLimitsLimitType,
    pub limit_max_amount: Option<i64>,
    pub limit_max_count: Option<i64>,
    pub limit_currency: Option<Value>,
    pub limit_effective_from: Option<DateTime<FixedOffset>>,
    pub limit_effective_to: Option<DateTime<FixedOffset>>,
}

impl From<TransactionCheckerFlat> for TransactionCheckerRow {
    fn from(value: TransactionCheckerFlat) -> Self {
        Self {
            channel: TransactionChannelSummary {
                id: value.id.to_string(),
                institution_id: value.institution_id.to_string(),
                channel_name: value.channel_name,
                channel_code: value.channel_code,
                requires_maker_checker: value.requires_maker_checker,
                metadata: value.metadata,
            },

            limit: value.limit_id.and_then(|id| {
                Some(TransactionLimitSummary {
                    id: id.to_string(),
                    institution_id: value.limit_institution_id.to_string(),
                    transaction_channel_id: value.limit_transaction_channel_id.to_string(),
                    account_category_id: value.limit_account_category_id.to_string(),
                    customer_type: value.limit_customer_type,
                    limit_type: value.limit_limit_type,
                    max_amount: value.limit_max_amount,
                    max_count: value.limit_max_count,
                    currency: value.limit_currency,
                    effective_from: value.limit_effective_from,
                    effective_to: value.limit_effective_to,
                })
            }),
        }
    }
}

// ===========================================
// End Transaction Checker
// ===========================================
//
// ===========================================
// Transaction Limit
// ===========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionLimitRow {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub customer_type: CustomerType,
    pub limit_type: TransactionLimitsLimitType,
    pub max_amount: Option<i64>,
    pub max_count: Option<i32>,
    pub currency: Option<Value>,
    pub is_active: Option<bool>,
    pub effective_from: Option<DateTime<FixedOffset>>,
    pub effective_to: Option<DateTime<FixedOffset>>,
    pub kyc_tier: Option<Value>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,

    pub transaction_channel: TransactionChannelSummary,
    pub account_category: AccountCategorySummary,
}

#[derive(Debug, FromQueryResult, Clone)]
pub struct TransactionLimitFlat {
    pub id: i64,
    pub institution_id: i64,
    pub customer_type: CustomerType,
    pub limit_type: TransactionLimitsLimitType,
    pub max_amount: Option<i64>,
    pub max_count: Option<i32>,
    pub currency: Option<Value>,
    pub is_active: Option<bool>,
    pub effective_from: Option<DateTime<FixedOffset>>,
    pub effective_to: Option<DateTime<FixedOffset>>,
    pub kyc_tier: Option<Value>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,

    pub transaction_channel_id: i64,
    pub channel_institution_id: i64,
    pub channel_name: Option<String>,
    pub channel_code: Option<String>,
    pub requires_maker_checker: Option<bool>,
    pub metadata: Option<Value>,

    pub account_category_id: i64,
    pub category_name: Option<String>,
    pub category_type: Option<String>,
    pub category_description: Option<String>,
    pub category_is_active: Option<bool>,
}

impl From<TransactionLimitFlat> for TransactionLimitRow {
    fn from(flat: TransactionLimitFlat) -> Self {
        Self {
            id: flat.id.to_string(),
            institution_id: flat.institution_id.to_string(),
            customer_type: flat.customer_type,
            limit_type: flat.limit_type,
            max_amount: flat.max_amount,
            max_count: flat.max_count,
            currency: flat.currency,
            is_active: flat.is_active,
            effective_from: flat.effective_from,
            effective_to: flat.effective_to,
            kyc_tier: flat.kyc_tier,
            created_at: flat.created_at,
            updated_at: flat.updated_at,

            transaction_channel: TransactionChannelSummary {
                id: flat.transaction_channel_id.to_string(),
                institution_id: flat.channel_institution_id.to_string(),
                channel_name: flat.channel_name,
                channel_code: flat.channel_code,
                requires_maker_checker: flat.requires_maker_checker,
                metadata: flat.metadata,
            },

            account_category: AccountCategorySummary {
                id: flat.account_category_id.to_string(),
                name: flat.category_name,
                category_type: flat.category_type,
                description: flat.category_description,
                is_active: flat.category_is_active,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRow {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub transaction_channel_id: String,
    pub transaction_reference: Option<String>,
    pub reversal_reason: Option<String>,
    pub debit_account_id: Option<String>,
    pub credit_account_id: Option<String>,
    pub debit_customer_id: Option<String>,
    pub credit_customer_id: Option<String>,
    pub amount: Decimal,
    pub currency: Option<Value>,
    pub fee_amount: Option<Decimal>,
    pub vat_amount: Option<Decimal>,
    pub total_amount: Option<Decimal>,
    pub transaction_group_id: uuid::Uuid,
    pub transaction_type: TransactionType,
    pub transaction_category: TransactionCategoryType,
    pub description: Option<String>,
    pub narrative: Option<String>,
    pub external_reference: Option<String>,
    pub status: TransactionStatus,
    pub posted_at: Option<DateTime<FixedOffset>>,
    pub completed_at: Option<DateTime<FixedOffset>>,
    pub failed_at: Option<DateTime<FixedOffset>>,
    pub failure_reason: Option<String>,
    pub value_date: Option<NaiveDate>,
    pub is_suspicious: Option<bool>,
    pub aml_alert_id: Option<String>,
    pub approved_at: Option<DateTime<FixedOffset>>,

    pub parent_transaction: Option<TransactionSummary>,
    pub cash_drawer: Option<TellerCashDrawerSummary>,
    pub approved_by: Option<StaffSummary>,
}

#[derive(Debug, FromQueryResult, Clone)]
pub struct TransactionFlat {
    pub id: i64,
    pub institution_id: i64,
    pub transaction_channel_id: i64,
    pub transaction_reference: Option<String>,
    pub reversal_reason: Option<String>,
    pub debit_account_id: Option<i64>,
    pub credit_account_id: Option<i64>,
    pub debit_customer_id: Option<i64>,
    pub credit_customer_id: Option<i64>,
    pub amount: i64,
    pub currency: Option<Value>,
    pub fee_amount: Option<i64>,
    pub vat_amount: Option<i64>,
    pub total_amount: Option<i64>,
    pub transaction_group_id: uuid::Uuid,
    pub transaction_type: TransactionType,
    pub transaction_category: TransactionCategoryType,
    pub description: Option<String>,
    pub narrative: Option<String>,
    pub external_reference: Option<String>,
    pub status: TransactionStatus,
    pub posted_at: Option<DateTime<FixedOffset>>,
    pub completed_at: Option<DateTime<FixedOffset>>,
    pub failed_at: Option<DateTime<FixedOffset>>,
    pub failure_reason: Option<String>,
    pub value_date: Option<NaiveDate>,
    pub is_suspicious: Option<bool>,
    pub aml_alert_id: Option<String>,
    pub approved_at: Option<DateTime<FixedOffset>>,

    pub parent_transaction_id: Option<i64>,
    pub parent_transaction_reference: Option<String>,
    pub parent_transaction_group_id: uuid::Uuid,
    pub parent_transaction_type: TransactionType,
    pub parent_transaction_category: TransactionCategoryType,
    pub parent_amount: i64,
    pub parent_currency: Option<Value>,
    pub parent_status: TransactionStatus,
    pub parent_posted_at: Option<DateTime<FixedOffset>>,
    pub parent_completed_at: Option<DateTime<FixedOffset>>,
    pub parent_failed_at: Option<DateTime<FixedOffset>>,

    pub cash_drawer_id: Option<i64>,
    pub drawer_status: Option<TellerCashDrawersStatus>,
    pub drawer_teller_id: i64,
    pub drawer_teller_name: String,
    pub drawer_teller_number: String,
    pub drawer_teller_branch_id: i64,
    pub drawer_supervisor_id: Option<i64>,
    pub drawer_employee_number: String,
    pub drawer_full_name: Option<String>,
    pub drawer_first_name: String,
    pub drawer_last_name: String,
    pub drawer_phone_number: String,
    pub drawer_email_address: String,
    pub drawer_job_title: Option<String>,
    pub drawer_department: Option<String>,
    pub drawer_employment_status: Option<StaffEmploymentEnum>,

    pub approved_by_id: Option<i64>,
    pub approved_employee_number: String,
    pub approved_full_name: Option<String>,
    pub approved_first_name: String,
    pub approved_last_name: String,
    pub approved_phone_number: String,
    pub approved_email_address: String,
    pub approved_job_title: Option<String>,
    pub approved_department: Option<String>,
    pub approved_employment_status: Option<StaffEmploymentEnum>,
}

impl TryFrom<TransactionFlat> for TransactionRow {
    type Error = conversions::MoneyError;

    fn try_from(flat: TransactionFlat) -> Result<Self, Self::Error> {
        let amount = conversions::major_conversion(flat.amount, "GHS");

        let fee_amount = flat
            .fee_amount
            .map(|fee| conversions::major_conversion(fee, "GHS"));

        let vat_amount = flat
            .vat_amount
            .map(|vat| conversions::major_conversion(vat, "GHS"));

        let total_amount = flat
            .total_amount
            .map(|tot| conversions::major_conversion(tot, "GHS"));

        let parent_amount = conversions::major_conversion(flat.parent_amount, "GHS");

        Ok(Self {
            id: flat.id.to_string(),
            institution_id: flat.institution_id.to_string(),
            transaction_channel_id: flat.transaction_channel_id.to_string(),
            transaction_reference: flat.transaction_reference,
            reversal_reason: flat.reversal_reason,
            debit_account_id: flat.debit_account_id.map(|did| did.to_string()),
            credit_account_id: flat.credit_account_id.map(|cid| cid.to_string()),
            debit_customer_id: flat.debit_customer_id.map(|dci| dci.to_string()),
            credit_customer_id: flat.credit_customer_id.map(|cci| cci.to_string()),
            amount,
            currency: flat.currency,
            fee_amount,
            vat_amount,
            total_amount,
            transaction_group_id: flat.transaction_group_id,
            transaction_type: flat.transaction_type,
            transaction_category: flat.transaction_category,
            description: flat.description,
            narrative: flat.narrative,
            external_reference: flat.external_reference,
            status: flat.status,
            posted_at: flat.posted_at,
            completed_at: flat.completed_at,
            failed_at: flat.failed_at,
            failure_reason: flat.failure_reason,
            value_date: flat.value_date,
            is_suspicious: flat.is_suspicious,
            aml_alert_id: flat.aml_alert_id,
            approved_at: flat.approved_at,

            parent_transaction: flat.parent_transaction_id.map(|id| TransactionSummary {
                id: id.to_string(),
                transaction_reference: flat.parent_transaction_reference,
                transaction_group_id: flat.transaction_group_id,
                transaction_type: flat.parent_transaction_type,
                transaction_category: flat.parent_transaction_category,
                amount: parent_amount,
                currency: flat.parent_currency,
                status: flat.parent_status,
                posted_at: flat.parent_posted_at,
                completed_at: flat.parent_completed_at,
                failed_at: flat.parent_failed_at,
            }),

            cash_drawer: flat.cash_drawer_id.map(|id| TellerCashDrawerSummary {
                id: id.to_string(),
                status: flat.drawer_status,
                teller: TellerSummary {
                    id: flat.drawer_teller_id.to_string(),
                    teller_name: flat.drawer_teller_name,
                    teller_number: flat.drawer_teller_number,
                    branch_id: flat.drawer_teller_branch_id.to_string(),
                },
                supervisor: flat.drawer_supervisor_id.map(|sid| StaffSummary {
                    id: sid.to_string(),
                    employee_number: flat.drawer_employee_number,
                    full_name: flat.drawer_full_name,
                    first_name: flat.drawer_first_name,
                    last_name: flat.drawer_last_name,
                    phone_number: flat.drawer_phone_number,
                    email_address: flat.drawer_email_address,
                    job_title: flat.drawer_job_title,
                    department: flat.drawer_department,
                    employment_status: flat.drawer_employment_status,
                }),
            }),

            approved_by: flat.approved_by_id.map(|id| StaffSummary {
                id: id.to_string(),
                employee_number: flat.approved_employee_number,
                full_name: flat.approved_full_name,
                first_name: flat.approved_first_name,
                last_name: flat.approved_last_name,
                phone_number: flat.approved_phone_number,
                email_address: flat.approved_email_address,
                job_title: flat.approved_job_title,
                department: flat.approved_department,
                employment_status: flat.approved_employment_status,
            }),
        })
    }
}
