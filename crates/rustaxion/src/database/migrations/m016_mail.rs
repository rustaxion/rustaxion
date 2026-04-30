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
                    .table(Mail::Table)
                    .if_not_exists()
                    .col(pk_auto(Mail::Id))
                    .col(integer(Mail::PlayerId))
                    .col(string(Mail::Title))
                    .col(text(Mail::Content))
                    // rewards stored as JSON: [{item_id, item_type, count}, ...]
                    .col(json(Mail::Rewards))
                    .col(boolean(Mail::IsGet).default(false))
                    .col(timestamp_with_time_zone(Mail::CreatedAt))
                    .col(timestamp_with_time_zone_null(Mail::ExpiresAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_mail-player_id")
                            .from_tbl(Mail::Table)
                            .from_col(Mail::PlayerId)
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
        manager.drop_table(Table::drop().table(Mail::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum Mail {
    Table,
    Id,
    PlayerId,
    Title,
    Content,
    Rewards,
    IsGet,
    CreatedAt,
    ExpiresAt,
}
