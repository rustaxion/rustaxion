use crate::{
    enums::comet::MainCmd,
    types::{packet::Packet, response::Response, session::SessionData},
};

mod gate;
mod login;
mod scene;

pub fn handle(session: &mut SessionData, packet: Packet) -> anyhow::Result<Vec<Response>> {
    match packet.main_cmd {
        MainCmd::Time | MainCmd::Select => gate::handle(session, packet),
        MainCmd::Login => login::handle(session, packet),
        MainCmd::Game => scene::handle(session, packet),
    }
}
