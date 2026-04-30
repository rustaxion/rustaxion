pub use sea_orm_migration::prelude::*;

mod m001_account;
mod m002_player;
mod m003_shop;
mod m004_daily_login;
mod m005_character;
mod m006_theme;
mod m007_player_character;
mod m008_player_theme;
mod m009_beatmap;
mod m010_player_beatmap;
mod m011_player_favourite_beatmap;
mod m012_score;
mod m013_account_credentials;
mod m014_player_guide_step;
mod m015_friend;
mod m016_mail;
mod m017_player_item;

pub struct Migrator;

#[rustfmt::skip]
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_account::Migration),
            Box::new(m002_player::Migration),
            Box::new(m003_shop::Migration),
            Box::new(m004_daily_login::Migration),
            Box::new(m005_character::Migration),
            Box::new(m006_theme::Migration),
            Box::new(m007_player_character::Migration),
            Box::new(m008_player_theme::Migration),
            Box::new(m009_beatmap::Migration),
            Box::new(m010_player_beatmap::Migration),
            Box::new(m011_player_favourite_beatmap::Migration),
            Box::new(m012_score::Migration),
            Box::new(m013_account_credentials::Migration),
            Box::new(m014_player_guide_step::Migration),
            Box::new(m015_friend::Migration),
            Box::new(m016_mail::Migration),
            Box::new(m017_player_item::Migration),
        ]
    }
}
