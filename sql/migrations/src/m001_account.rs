extern crate sea_orm_migration;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Account::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Account::Id).integer().not_null().auto_increment().primary_key()
                )
                .col(ColumnDef::new(Account::SteamId).string().not_null())
                .col(ColumnDef::new(Account::Token).string().not_null())
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Account::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    SteamId,
    Token,
}
