//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use super::sea_orm_active_enums::Country;
use super::sea_orm_active_enums::Language;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "player")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub account_id: i32,
    pub name: String,
    pub language: Language,
    pub country: Country,
    pub selected_character_id: i32,
    pub selected_theme_id: i32,
    pub head_id: i32,
    pub title_id: i32,
    pub total_score: i32,
    pub total4k_score: i32,
    pub total6k_score: i32,
    pub total8k_score: i32,
    pub level: i32,
    pub current_exp: i32,
    pub maximum_exp: i32,
    pub gold: i32,
    pub diamond: i32,
    pub current_stamina: i32,
    pub maximum_stamina: i32,
    pub honour_points: i32,
    pub total_arcade_score: i32,
    pub pre_rank: i32,
    pub pre_rank4k: i32,
    pub pre_rank6k: i32,
    pub pre_rank_param: i32,
    pub pre_rank4k_param: i32,
    pub pre_rank6k_param: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Account,
    #[sea_orm(
        belongs_to = "super::character::Entity",
        from = "Column::HeadId",
        to = "super::character::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Character2,
    #[sea_orm(
        belongs_to = "super::character::Entity",
        from = "Column::SelectedCharacterId",
        to = "super::character::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Character1,
    #[sea_orm(has_many = "super::daily_login::Entity")]
    DailyLogin,
    #[sea_orm(has_many = "super::player_beatmap::Entity")]
    PlayerBeatmap,
    #[sea_orm(has_many = "super::player_character::Entity")]
    PlayerCharacter,
    #[sea_orm(has_many = "super::player_favourite_beatmap::Entity")]
    PlayerFavouriteBeatmap,
    #[sea_orm(has_many = "super::player_theme::Entity")]
    PlayerTheme,
    #[sea_orm(has_many = "super::score::Entity")]
    Score,
    #[sea_orm(
        belongs_to = "super::theme::Entity",
        from = "Column::SelectedThemeId",
        to = "super::theme::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Theme,
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::daily_login::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DailyLogin.def()
    }
}

impl Related<super::player_beatmap::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlayerBeatmap.def()
    }
}

impl Related<super::player_character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlayerCharacter.def()
    }
}

impl Related<super::player_favourite_beatmap::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlayerFavouriteBeatmap.def()
    }
}

impl Related<super::player_theme::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlayerTheme.def()
    }
}

impl Related<super::score::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Score.def()
    }
}

impl Related<super::character::Entity> for Entity {
    fn to() -> RelationDef {
        super::player_character::Relation::Character.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::player_character::Relation::Player.def().rev())
    }
}

impl Related<super::theme::Entity> for Entity {
    fn to() -> RelationDef {
        super::player_theme::Relation::Theme.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::player_theme::Relation::Player.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
