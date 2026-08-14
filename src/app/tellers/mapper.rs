use crate::utils::models::{StaffSummary, TellerSummary};
use chrono::{DateTime, FixedOffset};
use entity::sea_orm_active_enums::{
    StaffEmploymentEnum, TellerCashDrawersStatus, TellerReconType, TellerStatus,
};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TellerRow {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub branch_id: String,
    pub teller_name: String,
    pub teller_number: String,
    pub drawer_limit: i64,
    pub current_drawer_balance: Option<i64>,
    pub status: Option<TellerStatus>,
    pub is_logged_in: Option<bool>,
    pub last_login_at: Option<DateTime<FixedOffset>>,
    pub current_session_id: Option<String>,
    pub current_terminal_id: Option<String>,

    pub staff: StaffSummary,
    pub supervisor: Option<StaffSummary>,
}

#[derive(Debug, FromQueryResult, Clone)]
pub struct TellerFlat {
    pub id: i64,
    pub institution_id: i64,
    pub branch_id: i64,
    pub teller_name: String,
    pub teller_number: String,
    pub drawer_limit: i64,
    pub current_drawer_balance: Option<i64>,
    pub status: Option<TellerStatus>,
    pub is_logged_in: Option<bool>,
    pub last_login_at: Option<DateTime<FixedOffset>>,
    pub current_session_id: Option<String>,
    pub current_terminal_id: Option<String>,

    pub staff_id: i64,
    pub staff_employee_number: String,
    pub staff_full_name: Option<String>,
    pub staff_first_name: String,
    pub staff_last_name: String,
    pub staff_phone_number: String,
    pub staff_email_address: String,
    pub staff_job_title: Option<String>,
    pub staff_department: Option<String>,
    pub staff_employment_status: Option<StaffEmploymentEnum>,

    pub supervisor_id: Option<i64>,
    pub supervisor_employee_number: String,
    pub supervisor_full_name: Option<String>,
    pub supervisor_first_name: String,
    pub supervisor_last_name: String,
    pub supervisor_phone_number: String,
    pub supervisor_email_address: String,
    pub supervisor_job_title: Option<String>,
    pub supervisor_department: Option<String>,
    pub supervisor_employment_status: Option<StaffEmploymentEnum>,
}

impl From<TellerFlat> for TellerRow {
    fn from(value: TellerFlat) -> Self {
        Self {
            id: value.id.to_string(),
            institution_id: value.institution_id.to_string(),
            branch_id: value.branch_id.to_string(),
            teller_name: value.teller_name,
            teller_number: value.teller_number,
            drawer_limit: value.drawer_limit,
            current_drawer_balance: value.current_drawer_balance,
            status: value.status,
            is_logged_in: value.is_logged_in,
            last_login_at: value.last_login_at,
            current_session_id: value.current_session_id,
            current_terminal_id: value.current_terminal_id,

            staff: StaffSummary {
                id: value.staff_id.to_string(),
                employee_number: value.staff_employee_number,
                full_name: value.staff_full_name,
                first_name: value.staff_first_name,
                last_name: value.staff_last_name,
                phone_number: value.staff_phone_number,
                email_address: value.staff_email_address,
                job_title: value.staff_job_title,
                department: value.staff_department,
                employment_status: value.staff_employment_status,
            },

            supervisor: value.supervisor_id.map(|id| StaffSummary {
                id: id.to_string(),
                employee_number: value.supervisor_employee_number,
                full_name: value.supervisor_full_name,
                first_name: value.supervisor_first_name,
                last_name: value.supervisor_last_name,
                phone_number: value.supervisor_phone_number,
                email_address: value.supervisor_email_address,
                job_title: value.supervisor_job_title,
                department: value.supervisor_department,
                employment_status: value.supervisor_employment_status,
            }),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TellerCashDrawerRow {
    #[serde(rename = "_id")]
    pub id: String,
    pub opening_cash_amount: Option<i64>,
    pub opening_cash: Option<Value>,
    pub total_cash_in: Option<i64>,
    pub total_cash_out: Option<i64>,
    pub cheque_count: Option<i32>,
    pub total_cheque_amount: Option<i64>,
    pub transfer_in_count: Option<i32>,
    pub total_transfer_in_amount: Option<i64>,
    pub transfer_out_count: Option<i32>,
    pub total_transfer_out_amount: Option<i64>,
    pub closing_balance: Option<i64>,
    pub closing_cash: Option<Value>,
    pub expected_amount: Option<i64>,
    pub variance_amount: Option<i64>,
    pub variance_reason: Option<String>,
    pub status: Option<TellerCashDrawersStatus>,
    pub opened_at: Option<DateTime<FixedOffset>>,
    pub closed_at: Option<DateTime<FixedOffset>>,

    pub teller: TellerSummary,
    pub supervisor: Option<StaffSummary>,
}

#[derive(Debug, FromQueryResult, Clone)]
pub struct TellerCashDrawerFlat {
    pub id: i64,
    pub opening_cash_amount: Option<i64>,
    pub opening_cash: Option<Value>,
    pub total_cash_in: Option<i64>,
    pub total_cash_out: Option<i64>,
    pub cheque_count: Option<i32>,
    pub total_cheque_amount: Option<i64>,
    pub transfer_in_count: Option<i32>,
    pub total_transfer_in_amount: Option<i64>,
    pub transfer_out_count: Option<i32>,
    pub total_transfer_out_amount: Option<i64>,
    pub closing_balance: Option<i64>,
    pub closing_cash: Option<Value>,
    pub expected_amount: Option<i64>,
    pub variance_amount: Option<i64>,
    pub variance_reason: Option<String>,
    pub status: Option<TellerCashDrawersStatus>,
    pub opened_at: Option<DateTime<FixedOffset>>,
    pub closed_at: Option<DateTime<FixedOffset>>,

    pub teller_id: i64,
    pub teller_name: String,
    pub teller_number: String,
    pub branch_id: i64,

    pub supervisor_id: Option<i64>,
    pub supervisor_employee_number: String,
    pub supervisor_full_name: Option<String>,
    pub supervisor_first_name: String,
    pub supervisor_last_name: String,
    pub supervisor_phone_number: String,
    pub supervisor_email_address: String,
    pub supervisor_job_title: Option<String>,
    pub supervisor_department: Option<String>,
    pub supervisor_employment_status: Option<StaffEmploymentEnum>,
}

impl From<TellerCashDrawerFlat> for TellerCashDrawerRow {
    fn from(value: TellerCashDrawerFlat) -> Self {
        Self {
            id: value.id.to_string(),
            opening_cash_amount: value.opening_cash_amount,
            opening_cash: value.opening_cash,
            total_cash_in: value.total_cash_in,
            total_cash_out: value.total_cash_out,
            cheque_count: value.cheque_count,
            total_cheque_amount: value.total_cheque_amount,
            transfer_in_count: value.transfer_in_count,
            total_transfer_in_amount: value.total_transfer_in_amount,
            transfer_out_count: value.transfer_out_count,
            total_transfer_out_amount: value.total_transfer_out_amount,
            closing_balance: value.closing_balance,
            closing_cash: value.closing_cash,
            expected_amount: value.expected_amount,
            variance_amount: value.variance_amount,
            variance_reason: value.variance_reason,
            status: value.status,
            opened_at: value.opened_at,
            closed_at: value.closed_at,

            teller: TellerSummary {
                id: value.teller_id.to_string(),
                teller_name: value.teller_name,
                teller_number: value.teller_number,
                branch_id: value.branch_id.to_string(),
            },

            supervisor: value.supervisor_id.map(|id| StaffSummary {
                id: id.to_string(),
                employee_number: value.supervisor_employee_number,
                full_name: value.supervisor_full_name,
                first_name: value.supervisor_first_name,
                last_name: value.supervisor_last_name,
                phone_number: value.supervisor_phone_number,
                email_address: value.supervisor_email_address,
                job_title: value.supervisor_job_title,
                department: value.supervisor_department,
                employment_status: value.supervisor_employment_status,
            }),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TellerReconRow {
    #[serde(rename = "_id")]
    pub id: String,
    pub cash_drawer_id: String,
    pub reconciliation_type: Option<TellerReconType>,
    pub notes: Option<String>,
    pub supervisor_id: Option<i64>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, FromQueryResult, Clone)]
pub struct TellerReconFlat {
    pub id: i64,
    pub cash_drawer_id: i64,
    pub reconciliation_type: Option<TellerReconType>,
    pub notes: Option<String>,
    pub supervisor_id: Option<i64>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl From<TellerReconFlat> for TellerReconRow {
    fn from(value: TellerReconFlat) -> Self {
        Self {
            id: value.id.to_string(),
            cash_drawer_id: value.cash_drawer_id.to_string(),
            reconciliation_type: value.reconciliation_type,
            notes: value.notes,
            supervisor_id: value.supervisor_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
