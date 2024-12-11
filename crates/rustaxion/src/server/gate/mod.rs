use crate::{
    enums::comet::{comet_gate::CometGate, MainCmd, ParaCmd},
    types::{packet::Packet, response::Response, session::SessionData},
};

mod create_character;
mod enter_game;
mod login_gate_verify;
mod time;

#[rustfmt::skip]
pub async fn handle(
    session: &mut SessionData,
    db: sea_orm::DatabaseConnection,
    Packet {
        main_cmd,
        para_cmd,
        data,
        ..
    }: Packet,
) -> anyhow::Result<Vec<Response>> {
    assert!(main_cmd == MainCmd::Time || main_cmd == MainCmd::Select);
    let ParaCmd::CometGate(para_cmd) = para_cmd else {
        anyhow::bail!("How did we get here?")
    };

    match para_cmd {
        CometGate::RequestUserGameTime => time::handle(session, db, data).await,
        CometGate::ResponseUserGameTime => time::handle(session, db, data).await,
        CometGate::LoginGateVerify => login_gate_verify::handle(session, db, data).await,
        CometGate::CreateCharacter => create_character::handle(session, db, data).await,
        CometGate::EnterGame => enter_game::handle(session, db, data).await,

        // NOTE(arjix): When given a client-side param, what should we do?
        _ => unreachable!()
    }
}
