use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251204_112805_create_institutions::Institutions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE staff_gender_enum AS ENUM ('MALE', 'FEMALE', 'OTHER')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE staff_employment_enum AS ENUM ('ACTIVE', 'INACTIVE')".to_string(),
            ))
            .await?;

        let branch = Table::create()
            .table(Branches::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Branches::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Branches::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Branches::Name).string())
            .col(ColumnDef::new(Branches::Code).string().unique_key())
            .col(ColumnDef::new(Branches::Address).json_binary())
            .col(ColumnDef::new(Branches::Phone).string())
            .col(ColumnDef::new(Branches::Email).string())
            .col(ColumnDef::new(Branches::Location).json_binary())
            .col(ColumnDef::new(Branches::IsMain).boolean().default(false))
            .col(
                ColumnDef::new(Branches::CashLimit)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(Branches::IsActive).boolean().default(false))
            .col(ColumnDef::new(Branches::Status).string())
            .col(ColumnDef::new(Branches::IsDeleted).boolean().default(false))
            .col(ColumnDef::new(Branches::DeletedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(Branches::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Branches::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Branches::Table, Branches::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(branch).await?;

        let staff_roles = Table::create()
            .table(StaffRoles::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(StaffRoles::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(StaffRoles::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(StaffRoles::RoleName).string().not_null())
            .col(ColumnDef::new(StaffRoles::RoleCode).string().not_null())
            .col(ColumnDef::new(StaffRoles::Description).string())
            .col(
                ColumnDef::new(StaffRoles::Permissions)
                    .json_binary()
                    .not_null(),
            )
            .col(ColumnDef::new(StaffRoles::IsAdmin).boolean().default(false))
            .col(
                ColumnDef::new(StaffRoles::IsSupervisor)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(StaffRoles::ReportsToRoleId).big_integer())
            .col(ColumnDef::new(StaffRoles::IsActive).boolean().default(true))
            .col(
                ColumnDef::new(StaffRoles::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(StaffRoles::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffRoles::Table, StaffRoles::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffRoles::Table, StaffRoles::ReportsToRoleId)
                    .to(StaffRoles::Table, StaffRoles::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_staff_role_code")
                    .table(StaffRoles::Table)
                    .col(StaffRoles::InstitutionId)
                    .col(StaffRoles::RoleCode)
                    .unique(),
            )
            .to_owned();

        manager.create_table(staff_roles).await?;

        let staff = Table::create()
            .table(Staff::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Staff::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Staff::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Staff::BranchId).big_integer())
            .col(
                ColumnDef::new(Staff::EmployeeNumber)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(Staff::FirstName).string().not_null())
            .col(ColumnDef::new(Staff::LastName).string().not_null())
            .col(ColumnDef::new(Staff::FullName).string())
            .col(ColumnDef::new(Staff::PhoneCountryCode).string())
            .col(ColumnDef::new(Staff::PhoneNumber).string().not_null())
            .col(ColumnDef::new(Staff::EmailAddress).string().not_null())
            .col(ColumnDef::new(Staff::DateOfBirth).date())
            .col(ColumnDef::new(Staff::Gender).custom("staff_gender_enum"))
            .col(ColumnDef::new(Staff::Nationality).string())
            .col(ColumnDef::new(Staff::JobTitle).string())
            .col(ColumnDef::new(Staff::Department).string())
            .col(
                ColumnDef::new(Staff::EmploymentStatus)
                    .custom("staff_employment_enum")
                    .default("ACTIVE"),
            )
            .col(ColumnDef::new(Staff::DateHired).date())
            .col(ColumnDef::new(Staff::DateTerminated).date())
            .col(ColumnDef::new(Staff::TerminationReason).string())
            .col(ColumnDef::new(Staff::RoleId).big_integer())
            .col(ColumnDef::new(Staff::Permissions).json_binary())
            .col(ColumnDef::new(Staff::PasswordHash).string().not_null())
            .col(ColumnDef::new(Staff::PasswordLastChangedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Staff::IsMfaEnabled).boolean().default(true))
            .col(ColumnDef::new(Staff::MfaSecretEncrypted).string())
            .col(
                ColumnDef::new(Staff::FailedLoginAttempts)
                    .integer()
                    .default(0),
            )
            .col(ColumnDef::new(Staff::LockedUntil).timestamp_with_time_zone())
            .col(ColumnDef::new(Staff::PerformanceRating).decimal_len(5, 2))
            .col(ColumnDef::new(Staff::LastAppraisalDate).date())
            .col(ColumnDef::new(Staff::SupervisorId).big_integer())
            .col(ColumnDef::new(Staff::CustomFields).json_binary())
            .col(ColumnDef::new(Staff::DeletedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(Staff::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Staff::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Staff::Table, Staff::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Staff::Table, Staff::BranchId)
                    .to(Branches::Table, Branches::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Staff::Table, Staff::SupervisorId)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_staff_email")
                    .table(Staff::Table)
                    .col(Staff::EmailAddress)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("idx_staff_phone")
                    .table(Staff::Table)
                    .col(Staff::PhoneNumber)
                    .unique(),
            )
            .to_owned();

        manager.create_table(staff).await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Branches::Table)
                    .add_column(ColumnDef::new(Branches::ManagerId).big_integer())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .from_tbl(Branches::Table)
                            .from_col(Branches::ManagerId)
                            .to_tbl(Staff::Table)
                            .to_col(Staff::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(StaffRoles::Table)
                    .add_column(ColumnDef::new(StaffRoles::CreatedBy).big_integer())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .from_tbl(StaffRoles::Table)
                            .from_col(StaffRoles::CreatedBy)
                            .to_tbl(Staff::Table)
                            .to_col(Staff::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Branches::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(StaffRoles::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Staff::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Branches {
    Table,
    Id,
    InstitutionId,
    Name,
    Code,
    Address,
    Phone,
    Email,
    Location,
    IsMain,
    ManagerId,
    CashLimit,
    IsActive,
    Status,
    IsDeleted,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum StaffRoles {
    Table,
    Id,
    InstitutionId,
    RoleName,
    RoleCode,
    Description,
    Permissions,
    IsAdmin,
    IsSupervisor,
    ReportsToRoleId,
    IsActive,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Staff {
    Table,
    Id,
    InstitutionId,
    BranchId,
    EmployeeNumber,
    FirstName,
    LastName,
    FullName,
    PhoneCountryCode,
    PhoneNumber,
    EmailAddress,
    DateOfBirth,
    Gender,
    Nationality,
    JobTitle,
    Department,
    EmploymentStatus,
    DateHired,
    DateTerminated,
    TerminationReason,
    RoleId,
    Permissions,
    PasswordHash,
    PasswordLastChangedAt,
    IsMfaEnabled,
    MfaSecretEncrypted,
    FailedLoginAttempts,
    LockedUntil,
    PerformanceRating,
    LastAppraisalDate,
    SupervisorId,
    CustomFields,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
