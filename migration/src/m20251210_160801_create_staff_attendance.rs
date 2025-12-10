use sea_orm_migration::prelude::*;

use crate::m20251204_150208_create_branches::Staff;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let staff_attendance = Table::create()
            .table(StaffAttendance::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(StaffAttendance::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(StaffAttendance::StaffId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(StaffAttendance::AttendanceDate)
                    .date()
                    .not_null(),
            )
            .col(ColumnDef::new(StaffAttendance::CheckInAt).timestamp_with_time_zone())
            .col(ColumnDef::new(StaffAttendance::CheckOutAt).timestamp_with_time_zone())
            .col(ColumnDef::new(StaffAttendance::CheckInLocation).json_binary())
            .col(ColumnDef::new(StaffAttendance::CheckOutLocation).json_binary())
            .col(ColumnDef::new(StaffAttendance::Status).string())
            .col(ColumnDef::new(StaffAttendance::HoursWorked).decimal_len(10, 2))
            .col(ColumnDef::new(StaffAttendance::Notes).text())
            .col(ColumnDef::new(StaffAttendance::ApprovedBy).big_integer())
            .col(
                ColumnDef::new(StaffAttendance::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(StaffAttendance::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffAttendance::Table, StaffAttendance::StaffId)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffAttendance::Table, StaffAttendance::ApprovedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .index(
                Index::create()
                    .name("idx_staff_attendance_date")
                    .table(StaffAttendance::Table)
                    .col(StaffAttendance::StaffId)
                    .col(StaffAttendance::AttendanceDate)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("idx_staff_attendance_date")
                    .table(StaffAttendance::Table)
                    .col(StaffAttendance::AttendanceDate),
            )
            .to_owned();

        manager.create_table(staff_attendance).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffAttendance::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum StaffAttendance {
    Table,
    Id,
    StaffId,
    AttendanceDate,
    CheckInAt,
    CheckOutAt,
    CheckInLocation,
    CheckOutLocation,
    Status,
    HoursWorked,
    Notes,
    ApprovedBy,
    CreatedAt,
    UpdatedAt,
}
