use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251205_154503_create_accounts::Accounts,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE acc_link_type AS ENUM ('JOINT', 'TRUST', 'CORPORATE')".to_string(),
            ))
            .await?;

        let acc_links = Table::create()
            .table(AccountLinks::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AccountLinks::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AccountLinks::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccountLinks::PrimaryAccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccountLinks::LinkedAccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccountLinks::LinkType)
                    .custom("acc_link_type")
                    .not_null(),
            )
            .col(ColumnDef::new(AccountLinks::Relationship).string())
            .col(ColumnDef::new(AccountLinks::AuthorizedLimit).decimal_len(20, 4))
            .col(
                ColumnDef::new(AccountLinks::IsCreditAllowed)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(AccountLinks::IsDebitAllowed)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(AccountLinks::Status)
                    .custom("acc_type_status")
                    .default("ACTIVE"),
            )
            .col(ColumnDef::new(AccountLinks::CreatedBy).big_integer())
            .col(
                ColumnDef::new(AccountLinks::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AccountLinks::UpdatedBy)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AccountLinks::Table, AccountLinks::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AccountLinks::Table, AccountLinks::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AccountLinks::Table, AccountLinks::PrimaryAccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AccountLinks::Table, AccountLinks::LinkedAccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(acc_links).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccountLinks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AccountLinks {
    Table,
    Id,
    InstitutionId,
    PrimaryAccountId,
    LinkedAccountId,
    LinkType,
    Relationship,
    AuthorizedLimit,
    IsDebitAllowed,
    IsCreditAllowed,
    Status,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
}
