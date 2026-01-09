use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_152312_create_customers::Customers,
    m20251210_161623_create_field_officer_routes::FieldOfficerRoutes,
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
                "CREATE TYPE field_officer_visit_types AS ENUM ('COLLECTION', 'VERIFICATION', 'DISBURSEMENT', 'RECOVERY')"
                    .to_string(),
            ))
            .await?;

        let field_officer_visits = Table::create()
            .table(FieldOfficerVisits::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(FieldOfficerVisits::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(FieldOfficerVisits::RouteId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FieldOfficerVisits::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FieldOfficerVisits::VisitOrder)
                    .integer()
                    .not_null(),
            )
            .col(ColumnDef::new(FieldOfficerVisits::VisitType).custom("field_officer_visit_types"))
            .col(ColumnDef::new(FieldOfficerVisits::ScheduledTime).timestamp_with_time_zone())
            .col(ColumnDef::new(FieldOfficerVisits::ArrivalTime).timestamp_with_time_zone())
            .col(ColumnDef::new(FieldOfficerVisits::DepartureTime).timestamp_with_time_zone())
            .col(ColumnDef::new(FieldOfficerVisits::CustomerLocation).json_binary())
            .col(ColumnDef::new(FieldOfficerVisits::ActualVisitLocation).json_binary())
            .col(ColumnDef::new(FieldOfficerVisits::GeoAccuracyMeters).decimal_len(10, 2))
            .col(ColumnDef::new(FieldOfficerVisits::Purpose).string())
            .col(ColumnDef::new(FieldOfficerVisits::Outcome).string())
            .col(
                ColumnDef::new(FieldOfficerVisits::FollowUpRequired)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(FieldOfficerVisits::ExpectedAmount).decimal_len(20, 4))
            .col(ColumnDef::new(FieldOfficerVisits::CollectedAmount).decimal_len(20, 4))
            .col(ColumnDef::new(FieldOfficerVisits::Photos).json_binary())
            .col(ColumnDef::new(FieldOfficerVisits::Signatures).json_binary())
            .col(
                ColumnDef::new(FieldOfficerVisits::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FieldOfficerVisits::Table, FieldOfficerVisits::RouteId)
                    .to(FieldOfficerRoutes::Table, FieldOfficerRoutes::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FieldOfficerVisits::Table, FieldOfficerVisits::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(field_officer_visits).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FieldOfficerVisits::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum FieldOfficerVisits {
    Table,
    Id,
    RouteId,
    CustomerId,
    VisitOrder,
    VisitType,
    ScheduledTime,
    ArrivalTime,
    DepartureTime,
    CustomerLocation,
    ActualVisitLocation,
    GeoAccuracyMeters,
    Purpose,
    Outcome,
    FollowUpRequired,
    ExpectedAmount,
    CollectedAmount,
    Photos,
    Signatures,
    CreatedAt,
}
