use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    CREATE EXTENSION IF NOT EXISTS pg_trgm;
                "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    CREATE EXTENSION IF NOT EXISTS postgis;
                "#
                .to_string(),
            ))
            .await?;

        let table_meta = Table::create()
            .table(Countries::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Countries::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Countries::Slug)
                    .string_len(32)
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(Countries::Name).string().not_null())
            .col(ColumnDef::new(Countries::OfficialName).string())
            .col(ColumnDef::new(Countries::CapitalCity).string())
            .col(ColumnDef::new(Countries::Currency).json_binary())
            .col(ColumnDef::new(Countries::FlagUrl).string())
            .col(ColumnDef::new(Countries::CallCode).string().not_null())
            .col(ColumnDef::new(Countries::IsoCode).string().not_null())
            .col(ColumnDef::new(Countries::MoreData).json_binary())
            .col(
                ColumnDef::new(Countries::IsActive)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(Countries::IsDeleted)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(ColumnDef::new(Countries::DeletedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(Countries::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Countries::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();

        manager.create_table(table_meta).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    DROP EXTENSION IF NOT EXISTS pg_trgm;
                "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    DROP EXTENSION IF NOT EXISTS postgis;
                "#
                .to_string(),
            ))
            .await?;

        manager
            .drop_table(Table::drop().table(Countries::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Countries {
    Table,
    Id,
    Slug,
    Name,
    OfficialName,
    CapitalCity,
    Currency,
    FlagUrl,
    CallCode,
    IsoCode,
    IsActive,
    DeletedAt,
    MoreData,
    IsDeleted,
    CreatedAt,
    UpdatedAt,
}
