use sea_orm_migration::prelude::*;
use sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let uploads = Table::create()
            .table(FileUploads::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(FileUploads::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(FileUploads::Slug)
                    .string()
                    .unique_key()
                    .not_null(),
            )
            .col(ColumnDef::new(FileUploads::OwnerId).big_integer())
            .col(
                ColumnDef::new(FileUploads::UploadedBy)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(FileUploads::FileKey).string().not_null())
            .col(ColumnDef::new(FileUploads::FileName).string().not_null())
            .col(ColumnDef::new(FileUploads::FileSize).big_integer())
            .col(ColumnDef::new(FileUploads::MimeType).string())
            .col(ColumnDef::new(FileUploads::AssignedEntity).string())
            .col(ColumnDef::new(FileUploads::FileType).string())
            .col(
                ColumnDef::new(FileUploads::PresignedUrl)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FileUploads::UrlExpiresAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(FileUploads::UploadedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(FileUploads::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();

        manager.create_table(uploads).await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    CREATE INDEX idx_file_uploads_user_id ON file_uploads(owner_id);
                "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    CREATE INDEX idx_file_uploads_presign ON file_uploads(presigned_url);
                "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    CREATE INDEX idx_file_uploaded_by ON file_uploads(uploaded_by);
                "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    CREATE INDEX idx_file_uploads_status ON file_uploads(uploaded_at) WHERE uploaded_at IS NULL;
                "#
                .to_string(),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FileUploads::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum FileUploads {
    Table,
    Id,
    Slug,
    OwnerId,
    UploadedBy,
    FileKey,
    FileName,
    FileSize,
    MimeType,
    FileType,
    AssignedEntity,
    PresignedUrl,
    UrlExpiresAt,
    UploadedAt,
    CreatedAt,
}
