extern crate sea_orm_migration;
pub use sea_orm_migration::prelude::*;

mod m001_account;
mod m002_player;

pub struct Migrator;

#[rustfmt::skip]
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_account::Migration),
            Box::new(m002_player::Migration)
        ]
    }
}
