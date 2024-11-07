use sea_orm_migration::{prelude::*, schema::*};

use super::{m002_player::Player, m009_beatmap::Beatmap};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlayerBeatmap::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("idx-player_beatmap")
                            .col(PlayerBeatmap::PlayerId)
                            .col(PlayerBeatmap::BeatmapId),
                    )
                    .col(integer(PlayerBeatmap::PlayerId))
                    .col(integer(PlayerBeatmap::BeatmapId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_beatmap-player_id")
                            .from_tbl(PlayerBeatmap::Table)
                            .from_col(PlayerBeatmap::PlayerId)
                            .to_tbl(Player::Table)
                            .to_col(Player::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_beatmap-beatmap_id")
                            .from_tbl(PlayerBeatmap::Table)
                            .from_col(PlayerBeatmap::BeatmapId)
                            .to_tbl(Beatmap::Table)
                            .to_col(Beatmap::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlayerBeatmap::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PlayerBeatmap {
    Table,
    PlayerId,
    BeatmapId,
}
