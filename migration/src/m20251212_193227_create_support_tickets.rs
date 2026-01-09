use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_152312_create_customers::Customers,
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
                "CREATE TYPE support_tickets_category AS ENUM ('TRANSACTION_ISSUE', 'LOAN_QUERY', 'ACCOUNT_QUERY')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE support_tickets_priority AS ENUM ('LOW', 'MEDIUM', 'HIGH', 'CRITICAL')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE support_tickets_status AS ENUM ('OPEN', 'IN_PROGRESS', 'WAITING_CUSTOMER', 'RESOLVED', 'CLOSED')"
                    .to_string(),
            ))
            .await?;

        let support_tickets = Table::create()
            .table(SupportTickets::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SupportTickets::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(SupportTickets::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SupportTickets::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SupportTickets::TicketNumber)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(SupportTickets::TicketSubject)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SupportTickets::TicketDescription)
                    .text()
                    .not_null(),
            )
            .col(ColumnDef::new(SupportTickets::TicketCategory).custom("support_tickets_category"))
            .col(ColumnDef::new(SupportTickets::TicketSubcategory).string())
            .col(
                ColumnDef::new(SupportTickets::Priority)
                    .custom("support_tickets_priority")
                    .default("MEDIUM"),
            )
            .col(ColumnDef::new(SupportTickets::AssignedTo).big_integer())
            .col(ColumnDef::new(SupportTickets::AssignedGroup).string())
            .col(
                ColumnDef::new(SupportTickets::Status)
                    .custom("support_tickets_status")
                    .default("OPEN"),
            )
            .col(
                ColumnDef::new(SupportTickets::IsEscalated)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(SupportTickets::EscalationLevel)
                    .integer()
                    .default(0),
            )
            .col(ColumnDef::new(SupportTickets::ResolutionSummary).text())
            .col(ColumnDef::new(SupportTickets::ResolutionTimeMinutes).integer())
            .col(ColumnDef::new(SupportTickets::ResolvedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(SupportTickets::ResolvedBy).big_integer())
            .col(ColumnDef::new(SupportTickets::CustomerRating).integer())
            .col(ColumnDef::new(SupportTickets::CustomerFeedback).text())
            .col(
                ColumnDef::new(SupportTickets::SlaBreach)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(SupportTickets::SlaDeadline).timestamp_with_time_zone())
            .col(
                ColumnDef::new(SupportTickets::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(SupportTickets::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(SupportTickets::Table, SupportTickets::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(SupportTickets::Table, SupportTickets::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(SupportTickets::Table, SupportTickets::AssignedTo)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(SupportTickets::Table, SupportTickets::ResolvedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.create_table(support_tickets).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SupportTickets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SupportTickets {
    Table,
    Id,
    InstitutionId,
    CustomerId,
    TicketNumber,
    TicketSubject,
    TicketDescription,
    TicketCategory,
    TicketSubcategory,
    Priority,
    AssignedTo,
    AssignedGroup,
    Status,
    IsEscalated,
    EscalationLevel,
    ResolutionSummary,
    ResolutionTimeMinutes,
    ResolvedAt,
    ResolvedBy,
    CustomerRating,
    CustomerFeedback,
    SlaBreach,
    SlaDeadline,
    CreatedAt,
    UpdatedAt,
}
