extern crate sea_orm_migration;
pub use sea_orm_migration::prelude::*;

mod m001_account;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m001_account::Migration)]
    }
}
