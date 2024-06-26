use sea_orm_migration::prelude::*;
use super::m002_player::Player;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // prettier-ignore
        manager.create_table(
            Table::create()
                .table(DailyLogin::Table)
                .if_not_exists()
                .col(ColumnDef::new(DailyLogin::Id).integer().auto_increment().not_null().primary_key())
                .col(ColumnDef::new(DailyLogin::PlayerId).big_integer().not_null())
                .col(ColumnDef::new(DailyLogin::LoginStreak).integer().not_null().default(0))
                .col(ColumnDef::new(DailyLogin::LoginCounter).integer().not_null().default(0))
                .col(ColumnDef::new(DailyLogin::LastDayLogin).big_unsigned().not_null().default(0))
                .foreign_key(ForeignKey::create()
                    .name("FK_DailyLogin_PlayerId")
                    .from(DailyLogin::Table, DailyLogin::PlayerId)
                    .to(Player::Table, Player::CharacterId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                )
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(DailyLogin::Table).to_owned()).await
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
