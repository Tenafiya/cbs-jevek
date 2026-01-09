use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let customer_app_preferences = Table::create()
            .table(CustomerAppPreferences::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CustomerAppPreferences::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(
                ColumnDef::new(CustomerAppPreferences::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CustomerAppPreferences::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CustomerAppPreferences::LanguageCode)
                    .string()
                    .default("en"),
            )
            .col(ColumnDef::new(CustomerAppPreferences::CurrencyCode).string())
            .col(
                ColumnDef::new(CustomerAppPreferences::Timezone)
                    .string()
                    .default("UTC"),
            )
            .col(
                ColumnDef::new(CustomerAppPreferences::BiometricEnabled)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(CustomerAppPreferences::PinEnabled)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(CustomerAppPreferences::QuickAccessEnabled)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(CustomerAppPreferences::PushNotificationsEnabled)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(CustomerAppPreferences::MarketingNotificationsEnabled)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(CustomerAppPreferences::DashboardLayout).json_binary())
            .col(ColumnDef::new(CustomerAppPreferences::FavoriteAccounts).json_binary())
            .col(
                ColumnDef::new(CustomerAppPreferences::DataSharingConsent)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(CustomerAppPreferences::Consents).json_binary())
            .col(
                ColumnDef::new(CustomerAppPreferences::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CustomerAppPreferences::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CustomerAppPreferences::Table,
                        CustomerAppPreferences::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CustomerAppPreferences::Table,
                        CustomerAppPreferences::CustomerId,
                    )
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(customer_app_preferences).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(CustomerAppPreferences::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum CustomerAppPreferences {
    Table,
    Id,
    InstitutionId,
    CustomerId,
    LanguageCode,
    CurrencyCode,
    Timezone,
    BiometricEnabled,
    PinEnabled,
    QuickAccessEnabled,
    PushNotificationsEnabled,
    MarketingNotificationsEnabled,
    DashboardLayout,
    FavoriteAccounts,
    DataSharingConsent,
    Consents,
    CreatedAt,
    UpdatedAt,
}
