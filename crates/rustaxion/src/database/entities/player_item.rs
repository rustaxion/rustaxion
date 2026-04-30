use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "player_item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub player_id: i32,
    /// Game item identifier (matches proto item_id)
    pub item_id: i32,
    /// Numeric value of proto's eItemType enum
    pub item_type: i32,
    pub count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::PlayerId",
        to = "super::player::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Player,
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
