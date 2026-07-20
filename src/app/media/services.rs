use sea_orm::DbErr;

#[inline(always)]
fn get_file_entity(entity: &str) -> Result<&'static str, DbErr> {
    match entity {
        "USR" => Ok("customers"),
        _ => Err(DbErr::Custom(format!("Unknown entity: {}", entity))),
    }
}