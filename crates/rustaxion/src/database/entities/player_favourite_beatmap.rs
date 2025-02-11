//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "player_favourite_beatmap")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub player_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub beatmap_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::beatmap::Entity",
        from = "Column::BeatmapId",
        to = "super::beatmap::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Beatmap,
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::PlayerId",
        to = "super::player::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Player,
}

impl Related<super::beatmap::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Beatmap.def()
    }
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
