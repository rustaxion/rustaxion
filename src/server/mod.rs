use crate::{
    enums::comet::MainCmd,
    types::{packet::Packet, response::Response, session::SessionData},
};

mod gate;
mod login;
mod scene;

pub fn handle(
    session: &mut SessionData,
    db: sea_orm::DatabaseConnection,
    packet: Packet,
) -> anyhow::Result<Vec<Response>> {
    match packet.main_cmd {
        MainCmd::Time | MainCmd::Select => gate::handle(session, db, packet),
        MainCmd::Login => login::handle(session, db, packet),
        MainCmd::Game => scene::handle(session, db, packet),
    }
}
