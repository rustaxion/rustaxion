use super::m001_account::Account;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Account::Table)
                    .add_column(string_null(AccountExt::AccountName).unique_key())
                    .add_column(string_null(AccountExt::Mail))
                    .add_column(string_null(AccountExt::PasswordHash))
                    .add_column(boolean(AccountExt::IsGuest).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Account::Table)
                    .drop_column(AccountExt::AccountName)
                    .drop_column(AccountExt::Mail)
                    .drop_column(AccountExt::PasswordHash)
                    .drop_column(AccountExt::IsGuest)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum AccountExt {
    AccountName,
    Mail,
    PasswordHash,
    IsGuest,
}
