use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(User::Username).string().not_null().unique_key())
                .col(ColumnDef::new(User::Password).string().not_null())
                .col(ColumnDef::new(User::CreatedAt).date_time().not_null().default("now()"))
                .col(ColumnDef::new(User::UpdatedAt).date_time().not_null().default("now()"))
                .col(ColumnDef::new(User::ArchivedAt).date_time().null())
                .col(ColumnDef::new(User::Archived).boolean().not_null().default(false))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Password,
    CreatedAt,
    UpdatedAt,
    ArchivedAt,
    Archived,
}
