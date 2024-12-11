use super::m002_player::Player;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // prettier-ignore
        manager
            .create_table(
                Table::create()
                    .table(DailyLogin::Table)
                    .if_not_exists()
                    .col(pk_auto(DailyLogin::Id))
                    .col(integer(DailyLogin::PlayerId))
                    .col(integer(DailyLogin::LoginStreak).default(0))
                    .col(integer(DailyLogin::LoginCounter).default(0))
                    .col(timestamp_with_time_zone(DailyLogin::LastDayLogin))
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_DailyLogin_PlayerId")
                            .from(DailyLogin::Table, DailyLogin::PlayerId)
                            .to(Player::Table, Player::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DailyLogin::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DailyLogin {
    Table,
    Id,
    PlayerId,
    LoginStreak,
    LoginCounter,
    LastDayLogin,
}
