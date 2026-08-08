use actix_web::web;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, InsertResult};

use crate::{
    AppState,
    app::tellers::models::{AddDrawerModel, AddTellerModel, AddTellerReconModel},
    utils::gen_snow_ids,
};

pub async fn add_teller(
    model: &AddTellerModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::tellers::ActiveModel>, DbErr> {
    use entity::tellers::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let teller = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        teller_name: Set(data.teller_name),
        teller_number: Set(data.teller_number),
        staff_id: Set(data.staff_id),
        ..Default::default()
    };

    Entity::insert(teller).exec(state.pgdb.get_ref()).await
}

pub async fn add_recon(
    model: &AddTellerReconModel,
    trn: &DatabaseTransaction,
) -> Result<InsertResult<entity::teller_reconciliations::ActiveModel>, DbErr> {
    use entity::teller_reconciliations::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let recon = ActiveModel {
        id: Set(snowflake),
        cash_drawer_id: Set(data.cash_drawer_id),
        reconciliation_type: Set(data.recon_type),
        notes: Set(data.notes),
        supervisor_id: Set(data.supervisor_id),
        ..Default::default()
    };

    Entity::insert(recon).exec(trn).await
}

pub async fn open_drawer(
    model: &AddDrawerModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::teller_cash_drawers::ActiveModel>, DbErr> {
    use entity::teller_cash_drawers::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let drawer = ActiveModel {
        id: Set(snowflake),
        teller_id: Set(data.teller_id),
        opening_cash: Set(Some(data.opening_cash)),
        ..Default::default()
    };

    Entity::insert(drawer).exec(state.pgdb.get_ref()).await
}
