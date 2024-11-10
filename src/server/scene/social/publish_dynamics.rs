use crate::types::{response::Response, session::SessionData};

#[rustfmt::skip]
pub async fn handle(_session: &mut SessionData, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![])
}
