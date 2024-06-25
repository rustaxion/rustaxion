use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "player")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub account_id: u32,
    pub steam_id: String,
    pub token: String,
}

/// Cake relation
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Account,
}

impl ColumnTrait for Relation {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Account => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Account =>
                Entity::belongs_to(crate::account::Entity)
                    .from(Column::AccountId)
                    .to(crate::account::Column::Id)
                    .into(),
        }
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
        where C: ConnectionTrait
    {
        Ok(self)
    }
}
