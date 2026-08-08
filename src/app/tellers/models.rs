use entity::sea_orm_active_enums::TellerReconType;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AddTellerModel {
    pub institution_id: i64,
    pub branch_id: i64,
    pub teller_name: String,
    pub teller_number: String,
    pub staff_id: i64
}

#[derive(Debug, Clone)]
pub struct AddTellerReconModel {
    pub cash_drawer_id: i64,
    pub recon_type: Option<TellerReconType>,
    pub notes: Option<String>,
    pub supervisor_id: Option<i64>
}

#[derive(Debug, Clone)]
pub struct AddDrawerModel {
    pub teller_id: i64,
    pub opening_cash_amount: i64,
    pub opening_cash: Value
}