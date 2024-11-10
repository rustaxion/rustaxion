use sea_orm_migration::{prelude::*, schema::*};

use super::{m002_player::Player, m006_theme::Theme};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlayerTheme::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("idx-player_theme")
                            .col(PlayerTheme::PlayerId)
                            .col(PlayerTheme::ThemeId),
                    )
                    .col(integer(PlayerTheme::PlayerId))
                    .col(integer(PlayerTheme::ThemeId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_theme-player_id")
                            .from_tbl(PlayerTheme::Table)
                            .from_col(PlayerTheme::PlayerId)
                            .to_tbl(Player::Table)
                            .to_col(Player::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_theme-theme_id")
                            .from_tbl(PlayerTheme::Table)
                            .from_col(PlayerTheme::ThemeId)
                            .to_tbl(Theme::Table)
                            .to_col(Theme::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlayerTheme::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PlayerTheme {
    Table,
    PlayerId,
    ThemeId,
}
