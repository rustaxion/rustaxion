pub use sea_orm_migration::prelude::*;

mod m001_account;
mod m002_player;
mod m003_shop;
mod m004_daily_login;
mod m005_character;
mod m006_theme;
mod m007_player_character;
mod m008_player_theme;

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
        ]
    }
}
