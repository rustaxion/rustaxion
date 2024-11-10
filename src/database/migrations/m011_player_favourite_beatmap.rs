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
                    .table(PlayerFavouriteBeatmap::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("idx-player_favourite-beatmap")
                            .col(PlayerFavouriteBeatmap::PlayerId)
                            .col(PlayerFavouriteBeatmap::BeatmapId),
                    )
                    .col(integer(PlayerFavouriteBeatmap::PlayerId))
                    .col(integer(PlayerFavouriteBeatmap::BeatmapId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_favourite_beatmap-player_id")
                            .from_tbl(PlayerFavouriteBeatmap::Table)
                            .from_col(PlayerFavouriteBeatmap::PlayerId)
                            .to_tbl(Player::Table)
                            .to_col(Player::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_favourite_beatmap-beatmap_id")
                            .from_tbl(PlayerFavouriteBeatmap::Table)
                            .from_col(PlayerFavouriteBeatmap::BeatmapId)
                            .to_tbl(Beatmap::Table)
                            .to_col(Beatmap::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(PlayerFavouriteBeatmap::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum PlayerFavouriteBeatmap {
    Table,
    PlayerId,
    BeatmapId,
}
