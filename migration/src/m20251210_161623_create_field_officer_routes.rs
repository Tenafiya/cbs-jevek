use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
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
                "CREATE TYPE field_officer_routes_status AS ENUM ('PLANNED', 'IN_PROGRESS', 'COMPLETED', 'CANCELLED')"
                    .to_string(),
            ))
            .await?;

        let field_officer_routes = Table::create()
            .table(FieldOfficerRoutes::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(FieldOfficerRoutes::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(FieldOfficerRoutes::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FieldOfficerRoutes::StaffId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FieldOfficerRoutes::RouteName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FieldOfficerRoutes::RouteDate)
                    .date()
                    .not_null(),
            )
            .col(ColumnDef::new(FieldOfficerRoutes::PlannedStartTime).timestamp_with_time_zone())
            .col(ColumnDef::new(FieldOfficerRoutes::PlannedEndTime).timestamp_with_time_zone())
            .col(ColumnDef::new(FieldOfficerRoutes::ActualStartTime).timestamp_with_time_zone())
            .col(ColumnDef::new(FieldOfficerRoutes::ActualEndTime).timestamp_with_time_zone())
            .col(ColumnDef::new(FieldOfficerRoutes::RouteGeojson).json_binary())
            .col(ColumnDef::new(FieldOfficerRoutes::EstimatedDistanceKm).decimal_len(10, 2))
            .col(ColumnDef::new(FieldOfficerRoutes::ActualDistanceKm).decimal_len(10, 2))
            .col(
                ColumnDef::new(FieldOfficerRoutes::Status)
                    .custom("field_officer_routes_status")
                    .default("PLANNED"),
            )
            .col(
                ColumnDef::new(FieldOfficerRoutes::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(FieldOfficerRoutes::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FieldOfficerRoutes::Table, FieldOfficerRoutes::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FieldOfficerRoutes::Table, FieldOfficerRoutes::StaffId)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_field_officer_routes_staff")
                    .table(FieldOfficerRoutes::Table)
                    .col(FieldOfficerRoutes::StaffId),
            )
            .index(
                Index::create()
                    .name("idx_field_officer_routes_date")
                    .table(FieldOfficerRoutes::Table)
                    .col(FieldOfficerRoutes::RouteDate),
            )
            .index(
                Index::create()
                    .name("idx_field_officer_routes_status")
                    .table(FieldOfficerRoutes::Table)
                    .col(FieldOfficerRoutes::Status),
            )
            .to_owned();

        manager.create_table(field_officer_routes).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FieldOfficerRoutes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum FieldOfficerRoutes {
    Table,
    Id,
    InstitutionId,
    StaffId,
    RouteName,
    RouteDate,
    PlannedStartTime,
    PlannedEndTime,
    ActualStartTime,
    ActualEndTime,
    RouteGeojson,
    EstimatedDistanceKm,
    ActualDistanceKm,
    Status,
    CreatedAt,
    UpdatedAt,
}
