use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create currencies table
        let currencies_table = Table::create()
            .table(Currencies::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Currencies::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
            )
            .col(ColumnDef::new(Currencies::InstitutionId).big_integer().not_null())
            .col(
                ColumnDef::new(Currencies::Code)
                    .char_len(3)
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(Currencies::NumericCode)
                    .char_len(3)
                    .unique_key(),
            )
            .col(
                ColumnDef::new(Currencies::Name)
                    .string_len(100)
                    .not_null(),
            )
            .col(
                ColumnDef::new(Currencies::Symbol)
                    .string_len(10),
            )
            .col(
                ColumnDef::new(Currencies::NativeSymbol)
                    .string_len(10),
            )
            .col(
                ColumnDef::new(Currencies::DecimalPlaces)
                    .small_integer()
                    .not_null()
                    .default(2),
            )
            .col(
                ColumnDef::new(Currencies::RoundingMode)
                    .string_len(20)
                    .default("'half_even'"),
            )
            .col(
                ColumnDef::new(Currencies::IsFiat)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(Currencies::IsCrypto)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(
                ColumnDef::new(Currencies::CryptoType)
                    .string_len(20),
            )
            .col(
                ColumnDef::new(Currencies::ContractAddress)
                    .string_len(100),
            )
            .col(
                ColumnDef::new(Currencies::BlockchainNetwork)
                    .string_len(50),
            )
            .col(
                ColumnDef::new(Currencies::IsActive)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(Currencies::IsTradeable)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(Currencies::IsWithdrawable)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(Currencies::IsDepositable)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(Currencies::MinTransferAmount)
                    .big_integer(),
            )
            .col(
                ColumnDef::new(Currencies::MaxTransferAmount)
                    .big_integer(),
            )
            .col(
                ColumnDef::new(Currencies::TransferFeeFixed)
                    .big_integer(),
            )
            .col(
                ColumnDef::new(Currencies::TransferFeePercent)
                    .decimal_len(5, 4),
            )
            .col(
                ColumnDef::new(Currencies::DailyTransferLimit)
                    .big_integer(),
            )
            .col(
                ColumnDef::new(Currencies::CountryCode)
                    .char_len(2),
            )
            .col(
                ColumnDef::new(Currencies::Region)
                    .string_len(50),
            )
            .col(
                ColumnDef::new(Currencies::CentralBank)
                    .string_len(200),
            )
            .col(
                ColumnDef::new(Currencies::IssuingAuthority)
                    .string_len(200),
            )
            .col(
                ColumnDef::new(Currencies::IconUrl)
                    .text(),
            )
            .col(
                ColumnDef::new(Currencies::ColorHex)
                    .char_len(7),
            )
            .col(
                ColumnDef::new(Currencies::Metadata)
                    .json_binary(),
            )
            .col(
                ColumnDef::new(Currencies::SortOrder)
                    .integer()
                    .default(999),
            )
            .col(
                ColumnDef::new(Currencies::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Currencies::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Currencies::DeactivatedAt)
                    .timestamp_with_time_zone(),
            )
            .col(
                ColumnDef::new(Currencies::DeactivatedReason)
                    .text(),
            )
            .to_owned();

        manager.create_table(currencies_table).await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE currencies 
                ADD CONSTRAINT valid_crypto_type CHECK (
                    (is_crypto = false AND crypto_type IS NULL) OR 
                    (is_crypto = true AND crypto_type IS NOT NULL)
                )
                "#
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE currencies 
                ADD CONSTRAINT valid_contract_address CHECK (
                    (is_crypto = false AND contract_address IS NULL) OR 
                    (is_crypto = true)
                )
                "#
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE currencies 
                ADD CONSTRAINT valid_fee_structure CHECK (
                    (transfer_fee_fixed IS NULL AND transfer_fee_percent IS NULL) OR
                    (transfer_fee_fixed IS NOT NULL OR transfer_fee_percent IS NOT NULL)
                )
                "#
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_currencies_active ON currencies(is_active, is_tradeable) WHERE is_active = true"
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_currencies_type ON currencies(is_fiat, is_crypto, blockchain_network) WHERE is_crypto = true"
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_currencies_sort ON currencies(sort_order, code)"
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_currencies_metadata ON currencies USING GIN(metadata jsonb_path_ops)"
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Currencies::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Currencies {
    Table,
    Id,
    InstitutionId,
    Code,
    NumericCode,
    Name,
    Symbol,
    NativeSymbol,
    DecimalPlaces,
    RoundingMode,
    IsFiat,
    IsCrypto,
    CryptoType,
    ContractAddress,
    BlockchainNetwork,
    IsActive,
    IsTradeable,
    IsWithdrawable,
    IsDepositable,
    MinTransferAmount,
    MaxTransferAmount,
    TransferFeeFixed,
    TransferFeePercent,
    DailyTransferLimit,
    CountryCode,
    Region,
    CentralBank,
    IssuingAuthority,
    IconUrl,
    ColorHex,
    Metadata,
    SortOrder,
    CreatedAt,
    UpdatedAt,
    DeactivatedAt,
    DeactivatedReason,
}