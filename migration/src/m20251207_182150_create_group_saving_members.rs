use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_152312_create_customers::Customers, m20251207_103023_create_saving_goals::SavingGoals,
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
                "CREATE TYPE group_saving_member_role AS ENUM ('MEMBER', 'LEADER', 'ADMIN')"
                    .to_string(),
            ))
            .await?;

        let mems = Table::create()
            .table(GroupSavingMembers::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(GroupSavingMembers::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(GroupSavingMembers::GroupGoalId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GroupSavingMembers::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GroupSavingMembers::MemberRole)
                    .custom("group_saving_member_role")
                    .default("MEMBER"),
            )
            .col(ColumnDef::new(GroupSavingMembers::JoinedDate).date())
            .col(ColumnDef::new(GroupSavingMembers::CommittedAmount).decimal_len(20, 4))
            .col(
                ColumnDef::new(GroupSavingMembers::ActualContributed)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(GroupSavingMembers::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(GroupSavingMembers::ExitDate).date())
            .col(ColumnDef::new(GroupSavingMembers::ExitReason).string())
            .col(
                ColumnDef::new(GroupSavingMembers::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(GroupSavingMembers::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GroupSavingMembers::Table, GroupSavingMembers::GroupGoalId)
                    .to(SavingGoals::Table, SavingGoals::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GroupSavingMembers::Table, GroupSavingMembers::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(mems).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GroupSavingMembers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum GroupSavingMembers {
    Table,
    Id,
    GroupGoalId,
    CustomerId,
    MemberRole,
    JoinedDate,
    CommittedAmount,
    ActualContributed,
    IsActive,
    ExitDate,
    ExitReason,
    CreatedAt,
    UpdatedAt,
}
