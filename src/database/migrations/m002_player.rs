use sea_orm_migration::prelude::*;
use super::m001_account::Account;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Player::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Player::CharacterId)
                        .big_integer()
                        .primary_key()
                        .auto_increment()
                        .not_null()
                )
                .col(ColumnDef::new(Player::AccountId).integer().not_null())
                .col(ColumnDef::new(Player::Name).string_len(50).not_null())
                .col(ColumnDef::new(Player::Language).small_integer().not_null())
                .col(ColumnDef::new(Player::Country).small_integer().not_null())
                .col(ColumnDef::new(Player::SelectedCharacterId).integer().not_null())
                .col(ColumnDef::new(Player::SelectedThemeId).integer().not_null())
                .col(ColumnDef::new(Player::HeadId).integer().not_null())
                .col(ColumnDef::new(Player::TitleId).integer().not_null())
                .col(ColumnDef::new(Player::TotalScore).integer().not_null().default(0))
                .col(ColumnDef::new(Player::Total4kScore).integer().not_null().default(0))
                .col(ColumnDef::new(Player::Total6kScore).integer().not_null().default(0))
                .col(ColumnDef::new(Player::Total8kScore).integer().not_null().default(0))
                .col(ColumnDef::new(Player::Level).integer().not_null().default(1))
                .col(ColumnDef::new(Player::CurrentExp).integer().not_null().default(0))
                .col(ColumnDef::new(Player::MaximumExp).integer().not_null().default(0))
                .col(ColumnDef::new(Player::Gold).integer().not_null().default(0))
                .col(ColumnDef::new(Player::Diamond).integer().not_null().default(0))
                .col(ColumnDef::new(Player::CurrentStamina).integer().not_null().default(10))
                .col(ColumnDef::new(Player::MaximumStamina).integer().not_null().default(10))
                .col(ColumnDef::new(Player::HonourPoints).integer().not_null().default(0))
                .col(ColumnDef::new(Player::TotalArcadeScore).integer().not_null().default(0))
                .col(ColumnDef::new(Player::PreRank).integer().not_null().default(0))
                .col(ColumnDef::new(Player::PreRank4k).integer().not_null().default(0))
                .col(ColumnDef::new(Player::PreRank6k).integer().not_null().default(0))
                .col(ColumnDef::new(Player::PreRankParam).integer().not_null().default(0))
                .col(ColumnDef::new(Player::PreRank4kParam).integer().not_null().default(0))
                .col(ColumnDef::new(Player::PreRank6kParam).integer().not_null().default(0))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_player-account_id")
                        .from(Player::Table, Player::AccountId)
                        .to(Account::Table, Account::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                )
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Player::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum Player {
    Table,
    CharacterId,
    AccountId,
    Name,
    Language,
    Country,
    SelectedCharacterId,
    SelectedThemeId,
    HeadId,
    TitleId,
    Level,
    CurrentExp,
    MaximumExp,
    Gold,
    Diamond,
    CurrentStamina,
    MaximumStamina,
    HonourPoints,
    TotalScore,
    Total4kScore,
    Total6kScore,
    Total8kScore,
    TotalArcadeScore,
    PreRank,
    PreRank4k,
    PreRank6k,
    PreRankParam,
    PreRank4kParam,
    PreRank6kParam,
}
