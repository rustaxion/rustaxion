use crate::{
    enums::comet::MainCmd,
    types::{packet::Packet, response::Response},
};

mod gate;
mod login;
mod scene;

pub fn handle(packet: Packet) -> anyhow::Result<Response> {
    match packet.main_cmd {
        MainCmd::Time | MainCmd::Select => gate::handle(packet),
        MainCmd::Login => login::handle(packet),
        MainCmd::Game => scene::handle(packet),
    }
}
