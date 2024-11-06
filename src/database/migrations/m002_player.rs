use super::m001_account::Account;
use extension::postgres::Type;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("language"))
                    .values(Language::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("country"))
                    .values(Country::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .if_not_exists()
                    .col(pk_auto(Player::Id))
                    .col(integer(Player::AccountId))
                    .col(string_len(Player::Name, 50))
                    .col(enumeration(
                        Player::Language,
                        Alias::new("Language"),
                        Language::iter(),
                    ))
                    .col(enumeration(
                        Player::Country,
                        Alias::new("Country"),
                        Country::iter(),
                    ))
                    .col(integer(Player::SelectedCharacterId))
                    .col(integer(Player::SelectedThemeId))
                    .col(integer(Player::HeadId))
                    .col(integer(Player::TitleId))
                    .col(integer(Player::TotalScore).default(0))
                    .col(integer(Player::Total4kScore).default(0))
                    .col(integer(Player::Total6kScore).default(0))
                    .col(integer(Player::Total8kScore).default(0))
                    .col(integer(Player::Level).default(1))
                    .col(integer(Player::CurrentExp).default(0))
                    .col(integer(Player::MaximumExp).default(0))
                    .col(integer(Player::Gold).default(0))
                    .col(integer(Player::Diamond).default(0))
                    .col(integer(Player::CurrentStamina).default(10))
                    .col(integer(Player::MaximumStamina).default(10))
                    .col(integer(Player::HonourPoints).default(0))
                    .col(integer(Player::TotalArcadeScore).default(0))
                    .col(integer(Player::PreRank).default(0))
                    .col(integer(Player::PreRank4k).default(0))
                    .col(integer(Player::PreRank6k).default(0))
                    .col(integer(Player::PreRankParam).default(0))
                    .col(integer(Player::PreRank4kParam).default(0))
                    .col(integer(Player::PreRank6kParam).default(0))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player-account_id")
                            .from(Player::Table, Player::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Player {
    Table,
    Id,
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

#[derive(Iden, EnumIter)]
pub enum Language {
    English = 1,
    China = 2,
    Japan = 3,
    Traditional = 4,
}

#[derive(Iden, EnumIter)]
pub enum Country {
    China = 1,
    HongKong = 2,
    TaiWan = 3,
    Macao = 4,
    Japan = 5,
    SouthKorea = 6,
    ASEAN = 7,
    UnitedStates = 8,
    UnitedKingdom = 9,
    EU = 10,
    Other = 11,
    Alien = 12,
}
