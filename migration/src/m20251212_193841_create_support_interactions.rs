use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251212_193227_create_support_tickets::SupportTickets;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE ticket_interaction_type AS ENUM ('CUSTOMER_MESSAGE', 'STAFF_RESPONSE', 'INTERNAL_NOTE', 'SYSTEM_ACTION')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE ticket_interaction_created_type AS ENUM ('CUSTOMER', 'STAFF', 'SYSTEM')"
                    .to_string(),
            ))
            .await?;

        let ticket_interactions = Table::create()
            .table(TicketInteractions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(TicketInteractions::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(TicketInteractions::TicketId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TicketInteractions::InteractionType)
                    .custom("ticket_interaction_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(TicketInteractions::InteractionContent)
                    .text()
                    .not_null(),
            )
            .col(ColumnDef::new(TicketInteractions::CreatedBy).big_integer())
            .col(
                ColumnDef::new(TicketInteractions::CreatedByType)
                    .custom("ticket_interaction_created_type"),
            )
            .col(
                ColumnDef::new(TicketInteractions::IsInternal)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(TicketInteractions::Attachments).json_binary())
            .col(
                ColumnDef::new(TicketInteractions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(TicketInteractions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(TicketInteractions::Table, TicketInteractions::TicketId)
                    .to(SupportTickets::Table, SupportTickets::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(ticket_interactions).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TicketInteractions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TicketInteractions {
    Table,
    Id,
    TicketId,
    InteractionType,
    InteractionContent,
    CreatedBy,
    CreatedByType,
    IsInternal,
    Attachments,
    CreatedAt,
    UpdatedAt,
}
