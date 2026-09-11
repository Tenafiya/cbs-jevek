use std::fmt;

use crate::sea_orm_active_enums::TransactionType;

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Credit => "CREDIT",
            Self::Debit => "DEBIT",
        })
    }
}
