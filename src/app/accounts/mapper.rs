use chrono::{DateTime, FixedOffset};
use entity::sea_orm_active_enums::{AccTypeStatus, CustomerType};
use sea_orm::{FromQueryResult, prelude::Decimal};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::models::{AccountSummary, AccountTypeSummary, CustomerSummary};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRow {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub acccount_number: Option<String>,
    pub account_name: Option<String>,
    pub currency: Option<Value>,
    pub current_balance: Option<i64>,
    pub available_balance: Option<i64>,
    pub ledger_balance: Option<i64>,
    pub hold_balance: Option<i64>,
    pub status: Option<AccTypeStatus>,
    pub activation_date: Option<DateTime<FixedOffset>>,
    pub dormancy_date: Option<DateTime<FixedOffset>>,
    pub frozen_at: Option<DateTime<FixedOffset>>,
    pub frozen_reason: Option<String>,
    pub is_overdraft_allowable: Option<bool>,
    pub overdraft_limit: Option<i64>,
    pub overdraft_used: Option<i64>,
    pub tags: Option<Vec<String>>,

    pub parent_account: Option<AccountSummary>,

    pub account_type: AccountTypeSummary,
    pub customer: CustomerSummary
}

#[derive(FromQueryResult, Debug, Clone)]
pub struct AccountFlat {
    pub id: i64,
    pub institution_id: i64,
    pub acccount_number: Option<String>,
    pub account_name: Option<String>,
    pub currency: Option<Value>,
    pub current_balance: Option<i64>,
    pub available_balance: Option<i64>,
    pub ledger_balance: Option<i64>,
    pub hold_balance: Option<i64>,
    pub status: Option<AccTypeStatus>,
    pub activation_date: Option<DateTime<FixedOffset>>,
    pub dormancy_date: Option<DateTime<FixedOffset>>,
    pub frozen_at: Option<DateTime<FixedOffset>>,
    pub frozen_reason: Option<String>,
    pub is_overdraft_allowable: Option<bool>,
    pub overdraft_limit: Option<i64>,
    pub overdraft_used: Option<i64>,
    pub tags: Option<Vec<String>>,

    pub parent_account_id: Option<i64>,
    pub parent_account_acccount_number: Option<String>,
    pub parent_account_account_name: Option<String>,
    pub parent_account_currency: Option<Value>,
    pub parent_account_current_balance: Option<i64>,
    pub parent_account_available_balance: Option<i64>,
    pub parent_account_ledger_balance: Option<i64>,
    pub parent_account_hold_balance: Option<i64>,

    pub account_type_id: i64,
    pub account_type_institution_id: i64,
    pub account_type_name: Option<String>,
    pub account_type_code: Option<String>,
    pub account_type_description: Option<String>,
    pub account_type_minimum_balance: Option<i64>,
    pub account_type_maximum_balance: Option<i64>,
    pub account_type_interest_rate: Option<Decimal>,
    pub account_type_maintenance_fee: Option<i64>,
    pub account_type_withdrawal_fee: Option<i64>,

    pub customer_id: i64,
    pub customer_type: Option<CustomerType>,
    pub customer_number: Option<String>,
    pub customer_first_name: Option<String>,
    pub customer_last_name: Option<String>,
}

impl From<AccountFlat> for AccountRow {
    fn from(flat: AccountFlat) -> Self {
        Self {
            id: flat.id.to_string(),
            institution_id: flat.institution_id.to_string(),
            acccount_number: flat.acccount_number,
            account_name: flat.account_name,
            currency: flat.currency,
            current_balance: flat.current_balance,
            available_balance: flat.available_balance,
            ledger_balance: flat.ledger_balance,
            hold_balance: flat.hold_balance,
            status: flat.status,
            activation_date: flat.activation_date,
            dormancy_date: flat.dormancy_date,
            frozen_at: flat.frozen_at,
            frozen_reason: flat.frozen_reason,
            is_overdraft_allowable: flat.is_overdraft_allowable,
            overdraft_limit: flat.overdraft_limit,
            overdraft_used: flat.overdraft_used,
            tags: flat.tags,

            parent_account: flat.parent_account_id.and_then(|id| {
                Some(AccountSummary {
                    id: id.to_string(),
                    acccount_number: flat.parent_account_acccount_number,
                    account_name: flat.parent_account_account_name,
                    currency: flat.parent_account_currency,
                    current_balance: flat.parent_account_current_balance,
                    available_balance: flat.parent_account_available_balance,
                    ledger_balance: flat.parent_account_ledger_balance,
                    hold_balance: flat.parent_account_hold_balance,
                })
            }),

            account_type: AccountTypeSummary {
                id: flat.account_type_id.to_string(),
                institution_id: flat.account_type_institution_id.to_string(),
                name: flat.account_type_name,
                code: flat.account_type_code,
                description: flat.account_type_description,
                minimum_balance: flat.account_type_minimum_balance,
                maximum_balance: flat.account_type_maximum_balance,
                interest_rate: flat.account_type_interest_rate,
                maintenance_fee: flat.account_type_maintenance_fee,
                withdrawal_fee: flat.account_type_withdrawal_fee,
            },

            customer: CustomerSummary {
                id: flat.customer_id.to_string(),
                customer_type: flat.customer_type,
                customer_number: flat.customer_number,
                first_name: flat.customer_first_name,
                last_name: flat.customer_last_name,
            },
        }
    }
}