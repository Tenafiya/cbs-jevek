use actix_web::web;
use migration::{Migrator, MigratorTrait};
use sea_orm::DbErr;
use serde::Deserialize;
use validator::Validate;

use crate::AppState;

pub async fn activate_migrations(
    state: &web::Data<AppState>,
    direction: String,
    steps: Option<u32>,
) -> Result<(), DbErr> {
    match direction.as_str() {
        "UP" => {
            Migrator::up(state.pgdb.get_ref(), steps).await?;
            Ok(())
        }
        "DOWN" => {
            Migrator::down(state.pgdb.get_ref(), steps).await?;
            Ok(())
        }
        _ => Ok(()),
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct MigratorParams {
    #[validate(range(min = 1))]
    #[serde(default)]
    pub steps: Option<u32>,
    #[serde(default = "default_direct")]
    pub direct: String,
}

#[inline]
fn default_direct() -> String {
    "UP".to_owned()
}
