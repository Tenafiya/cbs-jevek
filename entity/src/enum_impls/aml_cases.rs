use crate::sea_orm_active_enums::{AmlCaseStatus, AmlCasesPriority};

impl Default for AmlCaseStatus {
    fn default() -> Self {
        AmlCaseStatus::Open
    }
}

impl Default for AmlCasesPriority {
    fn default() -> Self {
        AmlCasesPriority::Low
    }
}
