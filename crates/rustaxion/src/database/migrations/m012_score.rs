use super::{m002_player::Player, m009_beatmap::Beatmap};
use extension::postgres::Type;
use sea_orm::{DeriveActiveEnum, EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("beatmap_mode"))
                    .values(Mode::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("beatmap_difficulty"))
                    .values(Difficulty::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Score::Table)
                    .if_not_exists()
                    .col(pk_auto(Score::Id))
                    .col(integer(Score::BeatmapId))
                    .col(integer(Score::PlayerId))
                    .col(enumeration(
                        Score::Mode,
                        Alias::new("beatmap_mode"),
                        Mode::iter(),
                    ))
                    .col(enumeration(
                        Score::Difficulty,
                        Alias::new("beatmap_difficulty"),
                        Difficulty::iter(),
                    ))
                    .col(integer(Score::FinishLevel))
                    .col(integer(Score::Score))
                    .col(boolean(Score::IsFullCombo))
                    .col(boolean(Score::IsPerfect))
                    .col(integer(Score::MissCount))
                    .col(timestamp_with_time_zone(Score::SubmittedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_score-beatmap_id")
                            .from_tbl(Score::Table)
                            .from_col(Score::BeatmapId)
                            .to_tbl(Beatmap::Table)
                            .to_col(Beatmap::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_score-player_id")
                            .from_tbl(Score::Table)
                            .from_col(Score::PlayerId)
                            .to_tbl(Player::Table)
                            .to_col(Player::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Score::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Score {
    Table,
    Id,
    BeatmapId,
    PlayerId,
    Mode,
    Difficulty,
    FinishLevel,
    Score,
    IsFullCombo,
    IsPerfect,
    MissCount,
    SubmittedAt,
}

#[derive(DeriveIden, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "beatmap_mode")]
pub enum Mode {
    #[sea_orm(string_value = "4k")]
    FourKeys = 1,
    #[sea_orm(string_value = "6k")]
    SixKeys = 2,
    #[sea_orm(string_value = "8k")]
    EightKeys = 3,
}

#[derive(DeriveIden, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "beatmap_difficulty")]
pub enum Difficulty {
    #[sea_orm(string_value = "ez")]
    Easy = 1,
    #[sea_orm(string_value = "nm")]
    Normal = 2,
    #[sea_orm(string_value = "hd")]
    Hard = 3,
}
