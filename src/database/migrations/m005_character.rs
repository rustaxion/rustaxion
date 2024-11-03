use sea_orm_migration::{prelude::*, schema::*};

use super::m002_player::Player;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Character::Table)
                    .if_not_exists()
                    .col(pk_auto(Character::Id))
                    .col(string(Character::Name))
                    .col(string(Character::Description))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Player::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_player-selected_character_id")
                            .from_tbl(Player::Table)
                            .from_col(Player::SelectedCharacterId)
                            .to_tbl(Character::Table)
                            .to_col(Character::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // TODO: Seed data

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Player::Table)
                    .drop_foreign_key(Alias::new("fk_player-selected_character_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Character::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Character {
    Table,
    Id,
    Name,
    Description,
}
