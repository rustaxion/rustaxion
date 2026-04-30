use super::m002_player::Player;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Player::Table)
                    .add_column(integer(PlayerExt::GuideStep).default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Player::Table)
                    .drop_column(PlayerExt::GuideStep)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum PlayerExt {
    GuideStep,
}
