use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

//=================================================================
// Enums
//=================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionFieldClassify {
    Transaction,
    Account,
    Customer,
    SenderAccount,
    ReceiverAccount,
    Cheque,
    Cash,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ConditionValue {
    String(String),
    Integer(i64),
    Decimal(Decimal),
    Boolean(bool),
    Strings(Vec<String>),
    Integers(Vec<i64>),
    Decimals(Vec<Decimal>),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConditionOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    NotIn,
    Contains,
    StartsWith,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum ConditionField {
    #[serde(rename = "transaction.amount")]
    TransactionAmount,

    #[serde(rename = "transaction.currency")]
    TransactionCurrency,

    #[serde(rename = "transaction.type")]
    TransactionType,

    #[serde(rename = "transaction.channel")]
    TransactionChannel,

    #[serde(rename = "transaction.daily_total")]
    TransactionDailyTotal,

    #[serde(rename = "transaction.daily_count")]
    TransactionDailyCount,

    #[serde(rename = "transaction.monthly_total")]
    TransactionMonthlyTotal,

    #[serde(rename = "transaction.monthly_count")]
    TransactionMonthlyCount,

    #[serde(rename = "account.balance")]
    AccountBalance,

    #[serde(rename = "account.status")]
    AccountStatus,

    #[serde(rename = "account.age_days")]
    AccountAgeDays,

    #[serde(rename = "customer.risk_score")]
    CustomerRiskScore,

    #[serde(rename = "customer.country")]
    CustomerCountry,

    #[serde(rename = "customer.occupation")]
    CustomerOccupation,

    #[serde(rename = "customer.is_pep")]
    CustomerIsPep,

    #[serde(rename = "customer.is_sanctioned")]
    CustomerIsSanctioned,

    #[serde(rename = "customer.customer_type")]
    CustomerCustomerType,

    #[serde(rename = "customer.cash_deposit_count_24h")]
    CustomerCashDepositCount24h,

    #[serde(rename = "customer.cash_deposit_amount_24h")]
    CustomerCashDepositAmount24h,

    #[serde(rename = "customer.transaction_count_24h")]
    CustomerTransactionCount24h,

    #[serde(rename = "cash.total")]
    CashTotal,

    #[serde(rename = "cash.count")]
    CashCount,

    #[serde(rename = "cash.denominations")]
    CashDenominations,

    #[serde(rename = "cheque.total")]
    ChequeTotal,

    #[serde(rename = "cheque.count")]
    ChequeCount,

    #[serde(rename = "cheques.total")]
    ChequesTotal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionValueType {
    String,
    Integer,
    Decimal,
    Boolean,
    Strings,
    Integers,
    Decimals,
}

impl ConditionValue {
    pub fn value_type(&self) -> ConditionValueType {
        match self {
            Self::String(_) => ConditionValueType::String,
            Self::Integer(_) => ConditionValueType::Integer,
            Self::Decimal(_) => ConditionValueType::Decimal,
            Self::Boolean(_) => ConditionValueType::Boolean,
            Self::Strings(_) => ConditionValueType::Strings,
            Self::Integers(_) => ConditionValueType::Integers,
            Self::Decimals(_) => ConditionValueType::Decimals,
        }
    }
}

//=================================================================
// Checkers
//=================================================================

pub struct FieldDefinition {
    pub field: ConditionField,
    pub value_type: ConditionValueType,
    pub allowed_operators: &'static [ConditionOperator],
}

pub static FIELD_DEFINITIONS: &[FieldDefinition] = &[
    // ─────────────────────────────────────────────────────────────
    // Transaction
    // ─────────────────────────────────────────────────────────────
    FieldDefinition {
        field: ConditionField::TransactionAmount,
        value_type: ConditionValueType::Decimal,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::TransactionCurrency,
        value_type: ConditionValueType::String,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Ne,
            ConditionOperator::In,
            ConditionOperator::NotIn,
        ],
    },
    FieldDefinition {
        field: ConditionField::TransactionAmount,
        value_type: ConditionValueType::Integer,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Lt,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::TransactionType,
        value_type: ConditionValueType::String,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Ne,
            ConditionOperator::In,
            ConditionOperator::NotIn,
        ],
    },
    FieldDefinition {
        field: ConditionField::TransactionChannel,
        value_type: ConditionValueType::String,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Ne,
            ConditionOperator::In,
            ConditionOperator::NotIn,
        ],
    },
    FieldDefinition {
        field: ConditionField::TransactionDailyTotal,
        value_type: ConditionValueType::Decimal,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::TransactionDailyCount,
        value_type: ConditionValueType::Integer,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::TransactionMonthlyTotal,
        value_type: ConditionValueType::Decimal,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::TransactionMonthlyCount,
        value_type: ConditionValueType::Integer,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    // ─────────────────────────────────────────────────────────────
    // Account
    // ─────────────────────────────────────────────────────────────
    FieldDefinition {
        field: ConditionField::AccountBalance,
        value_type: ConditionValueType::Decimal,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::AccountStatus,
        value_type: ConditionValueType::String,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Ne,
            ConditionOperator::In,
            ConditionOperator::NotIn,
        ],
    },
    FieldDefinition {
        field: ConditionField::AccountAgeDays,
        value_type: ConditionValueType::Integer,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    // ─────────────────────────────────────────────────────────────
    // Customer
    // ─────────────────────────────────────────────────────────────
    FieldDefinition {
        field: ConditionField::CustomerRiskScore,
        value_type: ConditionValueType::Decimal,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::CustomerCountry,
        value_type: ConditionValueType::String,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Ne,
            ConditionOperator::In,
            ConditionOperator::NotIn,
        ],
    },
    FieldDefinition {
        field: ConditionField::CustomerOccupation,
        value_type: ConditionValueType::String,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Ne,
            ConditionOperator::In,
            ConditionOperator::NotIn,
        ],
    },
    FieldDefinition {
        field: ConditionField::CustomerIsPep,
        value_type: ConditionValueType::Boolean,
        allowed_operators: &[ConditionOperator::Eq, ConditionOperator::Ne],
    },
    FieldDefinition {
        field: ConditionField::CustomerIsSanctioned,
        value_type: ConditionValueType::Boolean,
        allowed_operators: &[ConditionOperator::Eq, ConditionOperator::Ne],
    },
    FieldDefinition {
        field: ConditionField::CustomerCustomerType,
        value_type: ConditionValueType::String,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Ne,
            ConditionOperator::In,
            ConditionOperator::NotIn,
        ],
    },
    FieldDefinition {
        field: ConditionField::CustomerCashDepositCount24h,
        value_type: ConditionValueType::Integer,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::CustomerCashDepositAmount24h,
        value_type: ConditionValueType::Decimal,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::CustomerTransactionCount24h,
        value_type: ConditionValueType::Integer,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    // ─────────────────────────────────────────────────────────────
    // Cash
    // ─────────────────────────────────────────────────────────────
    FieldDefinition {
        field: ConditionField::CashTotal,
        value_type: ConditionValueType::Decimal,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::CashCount,
        value_type: ConditionValueType::Integer,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::CashDenominations,
        value_type: ConditionValueType::Decimal,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    // ─────────────────────────────────────────────────────────────
    // Cheque
    // ─────────────────────────────────────────────────────────────
    FieldDefinition {
        field: ConditionField::ChequeTotal,
        value_type: ConditionValueType::Decimal,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::ChequeCount,
        value_type: ConditionValueType::Integer,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
    FieldDefinition {
        field: ConditionField::ChequesTotal,
        value_type: ConditionValueType::Decimal,
        allowed_operators: &[
            ConditionOperator::Eq,
            ConditionOperator::Gt,
            ConditionOperator::Gte,
            ConditionOperator::Lt,
            ConditionOperator::Lte,
        ],
    },
];

impl ConditionField {
    pub fn classify(self) -> ConditionFieldClassify {
        match self {
            Self::TransactionAmount
            | Self::TransactionType
            | Self::TransactionChannel
            | Self::TransactionDailyTotal
            | Self::TransactionDailyCount
            | Self::TransactionMonthlyTotal
            | Self::TransactionMonthlyCount
            | Self::TransactionCurrency => ConditionFieldClassify::Transaction,

            Self::AccountBalance | Self::AccountAgeDays | Self::AccountStatus => {
                ConditionFieldClassify::Account
            }

            Self::CustomerCountry
            | Self::CustomerOccupation
            | Self::CustomerIsPep
            | Self::CustomerIsSanctioned
            | Self::CustomerCustomerType
            | Self::CustomerCashDepositCount24h
            | Self::CustomerCashDepositAmount24h
            | Self::CustomerTransactionCount24h
            | Self::CustomerRiskScore => ConditionFieldClassify::Customer,

            Self::ChequeTotal | Self::ChequeCount | Self::ChequesTotal => {
                ConditionFieldClassify::Cheque
            }

            Self::CashTotal | Self::CashCount | Self::CashDenominations => {
                ConditionFieldClassify::Cash
            }
        }
    }
}
