// enum_impls/customer_type.rs
use crate::sea_orm_active_enums::CustomerType;
use std::fmt;

impl fmt::Display for CustomerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Individual => "INDIVIDUAL",
            Self::Sme => "SME",
            Self::Group => "GROUP",
            Self::Cooperative => "COOPERATIVE",
            Self::Corporate => "CORPORATE",
        })
    }
}

impl Default for CustomerType {
    fn default() -> Self {
        CustomerType::Individual
    }
}
