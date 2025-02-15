use std::sync::Arc;

use proto::{enums::comet::MainCmd, packet::Packet};
use tokio::sync::Mutex;

use crate::types::{response::Response, session::SessionData};

mod gate;
mod login;
mod scene;

pub async fn handle(
    session: Arc<Mutex<SessionData>>,
    db: sea_orm::DatabaseConnection,
    packet: Packet,
) -> anyhow::Result<Vec<Response>> {
    match packet.main_cmd {
        MainCmd::Time | MainCmd::Select => gate::handle(session, db, packet).await,
        MainCmd::Login => login::handle(session, db, packet).await,
        MainCmd::Game => scene::handle(session, db, packet).await,
    }
}
