use sea_orm_migration::{prelude::*, schema::*};

use super::{m002_player::Player, m005_character::Character};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlayerCharacter::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("idx-player_character")
                            .col(PlayerCharacter::PlayerId)
                            .col(PlayerCharacter::CharacterId),
                    )
                    .col(integer(PlayerCharacter::PlayerId))
                    .col(integer(PlayerCharacter::CharacterId))
                    .col(integer(PlayerCharacter::Level).default(1))
                    .col(integer(PlayerCharacter::Experience).default(0))
                    .col(integer(PlayerCharacter::PlayCount).default(0))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_character-player_id")
                            .from_tbl(PlayerCharacter::Table)
                            .from_col(PlayerCharacter::PlayerId)
                            .to_tbl(Player::Table)
                            .to_col(Player::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_character-character_id")
                            .from_tbl(PlayerCharacter::Table)
                            .from_col(PlayerCharacter::CharacterId)
                            .to_tbl(Character::Table)
                            .to_col(Character::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlayerCharacter::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PlayerCharacter {
    Table,
    PlayerId,
    CharacterId,
    Level,
    Experience,
    PlayCount,
}
