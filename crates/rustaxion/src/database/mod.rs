use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;

pub mod entities;
pub mod helpers;
pub mod json;
pub mod migrations;

pub async fn establish_connection() -> anyhow::Result<DatabaseConnection> {
    let postgres_host = env::var("POSTGRES_HOST").unwrap();
    let postgres_user = env::var("POSTGRES_USER").unwrap();
    let postgres_password = env::var("POSTGRES_PASSWORD").unwrap();
    let postgres_db = env::var("POSTGRES_DB").unwrap();

    let connection_uri = format!(
        "postgres://{}:{}@{}/{}",
        postgres_user, postgres_password, postgres_host, postgres_db
    );

    let opt = ConnectOptions::new(connection_uri)
        .sqlx_logging(false)
        .min_connections(1)
        .max_connections(1)
        .connect_lazy(false)
        .to_owned();

    let db = Database::connect(opt).await?;

    use migrations::MigratorTrait;
    migrations::Migrator::up(&db, None).await?;

    Ok(db)
}
