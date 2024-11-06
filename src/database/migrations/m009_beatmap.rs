use extension::postgres::Type;
use sea_orm::{ActiveModelTrait, EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

use crate::database;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("beatmap_kind"))
                    .values(BeatmapKind::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Beatmap::Table)
                    .if_not_exists()
                    .col(pk_auto(Beatmap::Id))
                    .col(string(Beatmap::Name))
                    .col(integer(Beatmap::Quality))
                    .col(string(Beatmap::Composer))
                    .col(enumeration(
                        Beatmap::Kind,
                        Alias::new("beatmap_kind"),
                        BeatmapKind::iter(),
                    ))
                    .col(integer_null(Beatmap::DLCPack))
                    .col(json_binary(Beatmap::BPMRange))
                    .col(json_binary(Beatmap::Key4))
                    .col(json_binary(Beatmap::Key6))
                    .col(json_binary(Beatmap::Key8))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        type BeatmapModel = database::entities::beatmap::ActiveModel;

        // TODO: Seed data
        let beatmaps: Vec<BeatmapModel> = vec![];
        for beatmap in beatmaps {
            beatmap.insert(db).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Beatmap::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Beatmap {
    Table,
    Id,
    Name,
    Quality,
    Composer,
    Kind,
    DLCPack, // TODO: Add a DLC pack table
    BPMRange,
    Key4,
    Key6,
    Key8,
}

#[derive(Iden, EnumIter)]
pub enum BeatmapKind {
    Tutorial,
    Free,
    Sale,
    DLC,
}
