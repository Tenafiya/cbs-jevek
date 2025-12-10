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
                "CREATE TYPE agent_kyc_docs_doctype AS ENUM ('ID', 'BUSINESS_CERT', 'UTILITY_BILL')"
                    .to_string(),
            ))
            .await?;

        let kyc_docs = Table::create()
            .table(AgentKycDocs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AgentKycDocs::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AgentKycDocs::AgentId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AgentKycDocs::DocumentType).custom("agent_kyc_docs_doctype"))
            .col(ColumnDef::new(AgentKycDocs::DocumentUrl).string())
            .col(
                ColumnDef::new(AgentKycDocs::VerificationStatus)
                    .custom("verification_status_type")
                    .default("PENDING"),
            )
            .col(ColumnDef::new(AgentKycDocs::VerifiedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(AgentKycDocs::RejectionReason).string())
            .col(
                ColumnDef::new(AgentKycDocs::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AgentKycDocs::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();

        manager.create_table(kyc_docs).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AgentKycDocs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AgentKycDocs {
    Table,
    Id,
    AgentId,
    DocumentType,
    DocumentUrl,
    VerificationStatus,
    VerifiedBy,
    VerifiedAt,
    RejectionReason,
    CreatedAt,
    UpdatedAt,
}
