use std::fmt;

use crate::sea_orm_active_enums::{
    TransactionCategoryType, TransactionLimitsLimitType, TransactionStatus, TransactionType,
};

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Credit => "CREDIT",
            Self::Debit => "DEBIT",
        })
    }
}

impl Default for TransactionType {
    fn default() -> Self {
        TransactionType::Credit
    }
}

impl Default for TransactionCategoryType {
    fn default() -> Self {
        TransactionCategoryType::CashDeposit
    }
}

impl Default for TransactionStatus {
    fn default() -> Self {
        TransactionStatus::Pending
    }
}

impl Default for TransactionLimitsLimitType {
    fn default() -> Self {
        TransactionLimitsLimitType::Daily
    }
}
