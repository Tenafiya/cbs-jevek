use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_152312_create_customers::Customers, m20251205_154503_create_accounts::Accounts,
    m20251206_143556_create_loan_applications::LoanApplications,
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
                "CREATE TYPE staff_task_priority AS ENUM ('LOW', 'MEDIUM', 'HIGH', 'CRITICAL')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE staff_task_status AS ENUM ('PENDING', 'IN_PROGRESS', 'COMPLETED', 'PAUSED', 'CANCELLED')".to_string(),
            ))
            .await?;

        let staff_tasks = Table::create()
            .table(StaffTasks::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(StaffTasks::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(StaffTasks::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(StaffTasks::TaskName).string().not_null())
            .col(ColumnDef::new(StaffTasks::TaskDescription).string())
            .col(
                ColumnDef::new(StaffTasks::AssignedTo)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(StaffTasks::AssignedBy)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(StaffTasks::CustomerId).big_integer())
            .col(ColumnDef::new(StaffTasks::LoanApplicationId).big_integer())
            .col(ColumnDef::new(StaffTasks::AccountId).big_integer())
            .col(ColumnDef::new(StaffTasks::DueDate).timestamp_with_time_zone())
            .col(
                ColumnDef::new(StaffTasks::Priority)
                    .custom("staff_task_priority")
                    .default("MEDIUM"),
            )
            .col(
                ColumnDef::new(StaffTasks::Status)
                    .custom("staff_task_status")
                    .default("PENDING"),
            )
            .col(
                ColumnDef::new(StaffTasks::CompletionPercentage)
                    .integer()
                    .default(0),
            )
            .col(ColumnDef::new(StaffTasks::CompletedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(StaffTasks::CompletedNotes).string())
            .col(
                ColumnDef::new(StaffTasks::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(StaffTasks::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffTasks::Table, StaffTasks::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffTasks::Table, StaffTasks::AssignedTo)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffTasks::Table, StaffTasks::AssignedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffTasks::Table, StaffTasks::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffTasks::Table, StaffTasks::LoanApplicationId)
                    .to(LoanApplications::Table, LoanApplications::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffTasks::Table, StaffTasks::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_staff_tasks_assigned_to")
                    .table(StaffTasks::Table)
                    .col(StaffTasks::AssignedTo),
            )
            .index(
                Index::create()
                    .name("idx_staff_tasks_status")
                    .table(StaffTasks::Table)
                    .col(StaffTasks::Status),
            )
            .index(
                Index::create()
                    .name("idx_staff_tasks_due_date")
                    .table(StaffTasks::Table)
                    .col(StaffTasks::DueDate),
            )
            .to_owned();

        manager.create_table(staff_tasks).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffTasks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum StaffTasks {
    Table,
    Id,
    InstitutionId,
    TaskName,
    TaskDescription,
    AssignedTo,
    AssignedBy,
    CustomerId,
    LoanApplicationId,
    AccountId,
    DueDate,
    Priority,
    Status,
    CompletionPercentage,
    CompletedAt,
    CompletedNotes,
    CreatedAt,
    UpdatedAt,
}
