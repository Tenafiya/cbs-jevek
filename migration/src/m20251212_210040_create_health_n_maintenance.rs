use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251204_150208_create_branches::Staff;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE health_status AS ENUM ('HEALTHY', 'UNHEALTHY')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE maintenance_window_status AS ENUM ('SCHEDULED', 'PENDING', 'COMPLETED', 'POSTPONED', 'CANCELLED')".to_string(),
            ))
            .await?;

        // System health metrics table
        let system_health_metrics = Table::create()
            .table(SystemHealthMetrics::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SystemHealthMetrics::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(SystemHealthMetrics::MetricName)
                    .string_len(255)
                    .not_null(),
            )
            .col(ColumnDef::new(SystemHealthMetrics::MetricCategory).string())
            .col(ColumnDef::new(SystemHealthMetrics::MetricValue).decimal_len(20, 4))
            .col(ColumnDef::new(SystemHealthMetrics::MetricUnit).string())
            .col(ColumnDef::new(SystemHealthMetrics::WarningThreshold).decimal_len(20, 4))
            .col(ColumnDef::new(SystemHealthMetrics::CriticalThreshold).decimal_len(20, 4))
            .col(
                ColumnDef::new(SystemHealthMetrics::MetricStatus)
                    .custom("health_status")
                    .default("HEALTHY"),
            )
            .col(ColumnDef::new(SystemHealthMetrics::Metadata).json_binary())
            .col(
                ColumnDef::new(SystemHealthMetrics::RecordedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .index(
                Index::create()
                    .name("idx_health_metrics_name")
                    .table(SystemHealthMetrics::Table)
                    .col(SystemHealthMetrics::MetricName),
            )
            .index(
                Index::create()
                    .name("idx_health_metrics_category")
                    .table(SystemHealthMetrics::Table)
                    .col(SystemHealthMetrics::MetricCategory),
            )
            .index(
                Index::create()
                    .name("idx_health_metrics_status")
                    .table(SystemHealthMetrics::Table)
                    .col(SystemHealthMetrics::MetricStatus),
            )
            .index(
                Index::create()
                    .name("idx_health_metrics_recorded_at")
                    .table(SystemHealthMetrics::Table)
                    .col(SystemHealthMetrics::RecordedAt),
            )
            .to_owned();

        manager.create_table(system_health_metrics).await?;

        // System maintenance windows table
        let system_maintenance_windows = Table::create()
            .table(SystemMaintenanceWindows::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SystemMaintenanceWindows::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(SystemMaintenanceWindows::WindowName)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(SystemMaintenanceWindows::WindowDescription).text())
            .col(
                ColumnDef::new(SystemMaintenanceWindows::ScheduledStartAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SystemMaintenanceWindows::ScheduledEndAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(SystemMaintenanceWindows::AffectedServices).json_binary())
            .col(
                ColumnDef::new(SystemMaintenanceWindows::NotificationSent)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(SystemMaintenanceWindows::WindowStatus)
                    .custom("maintenance_window_status")
                    .default("SCHEDULED"),
            )
            .col(ColumnDef::new(SystemMaintenanceWindows::ActualStartAt).timestamp_with_time_zone())
            .col(ColumnDef::new(SystemMaintenanceWindows::ActualEndAt).timestamp_with_time_zone())
            .col(ColumnDef::new(SystemMaintenanceWindows::CreatedBy).big_integer())
            .col(
                ColumnDef::new(SystemMaintenanceWindows::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(SystemMaintenanceWindows::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        SystemMaintenanceWindows::Table,
                        SystemMaintenanceWindows::CreatedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .index(
                Index::create()
                    .name("idx_maintenance_windows_status")
                    .table(SystemMaintenanceWindows::Table)
                    .col(SystemMaintenanceWindows::WindowStatus),
            )
            .index(
                Index::create()
                    .name("idx_maintenance_windows_scheduled")
                    .table(SystemMaintenanceWindows::Table)
                    .col(SystemMaintenanceWindows::ScheduledStartAt)
                    .col(SystemMaintenanceWindows::ScheduledEndAt),
            )
            .to_owned();

        manager.create_table(system_maintenance_windows).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(SystemMaintenanceWindows::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(SystemHealthMetrics::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SystemHealthMetrics {
    Table,
    Id,
    MetricName,
    MetricCategory,
    MetricValue,
    MetricUnit,
    WarningThreshold,
    CriticalThreshold,
    MetricStatus,
    Metadata,
    RecordedAt,
}

#[derive(DeriveIden)]
pub enum SystemMaintenanceWindows {
    Table,
    Id,
    WindowName,
    WindowDescription,
    ScheduledStartAt,
    ScheduledEndAt,
    AffectedServices,
    NotificationSent,
    WindowStatus,
    ActualStartAt,
    ActualEndAt,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
