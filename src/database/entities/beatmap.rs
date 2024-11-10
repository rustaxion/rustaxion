//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use super::sea_orm_active_enums::BeatmapKind;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "beatmap")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub quality: i32,
    pub composer: String,
    pub kind: BeatmapKind,
    pub dlc_pack: Option<i32>,
    #[sea_orm(column_type = "JsonBinary")]
    pub bpm_range: Json,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub key4: Option<Json>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub key6: Option<Json>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub key8: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::player_beatmap::Entity")]
    PlayerBeatmap,
    #[sea_orm(has_many = "super::player_favourite_beatmap::Entity")]
    PlayerFavouriteBeatmap,
    #[sea_orm(has_many = "super::score::Entity")]
    Score,
}

impl Related<super::player_beatmap::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlayerBeatmap.def()
    }
}

impl Related<super::player_favourite_beatmap::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlayerFavouriteBeatmap.def()
    }
}

impl Related<super::score::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Score.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
