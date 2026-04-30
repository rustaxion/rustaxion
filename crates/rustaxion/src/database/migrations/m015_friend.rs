use super::m002_player::Player;
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
                    .as_enum(Alias::new("friend_status"))
                    .values(FriendStatus::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Friend::Table)
                    .if_not_exists()
                    .col(pk_auto(Friend::Id))
                    .col(integer(Friend::PlayerId))
                    .col(integer(Friend::FriendId))
                    .col(enumeration(
                        Friend::Status,
                        Alias::new("friend_status"),
                        FriendStatus::iter(),
                    ))
                    .col(timestamp_with_time_zone(Friend::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_friend-player_id")
                            .from_tbl(Friend::Table)
                            .from_col(Friend::PlayerId)
                            .to_tbl(Player::Table)
                            .to_col(Player::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_friend-friend_id")
                            .from_tbl(Friend::Table)
                            .from_col(Friend::FriendId)
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
        manager.drop_table(Table::drop().table(Friend::Table).to_owned()).await?;
        manager
            .drop_type(extension::postgres::Type::drop().name(Alias::new("friend_status")).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Friend {
    Table,
    Id,
    PlayerId,
    FriendId,
    Status,
    CreatedAt,
}

#[derive(Iden, EnumIter)]
pub enum FriendStatus {
    Pending = 1,
    Accepted = 2,
}
