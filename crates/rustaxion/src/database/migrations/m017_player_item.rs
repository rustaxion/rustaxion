use super::m002_player::Player;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlayerItem::Table)
                    .if_not_exists()
                    .col(pk_auto(PlayerItem::Id))
                    .col(integer(PlayerItem::PlayerId))
                    // item_id is the game's item identifier (e.g. head icon id, title id, etc.)
                    .col(integer(PlayerItem::ItemId))
                    // item_type mirrors proto's eItemType numeric value
                    .col(integer(PlayerItem::ItemType))
                    .col(integer(PlayerItem::Count).default(1))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_item-player_id")
                            .from_tbl(PlayerItem::Table)
                            .from_col(PlayerItem::PlayerId)
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
        manager.drop_table(Table::drop().table(PlayerItem::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum PlayerItem {
    Table,
    Id,
    PlayerId,
    ItemId,
    ItemType,
    Count,
}
