use crate::{
    enums::comet::{comet_gate::CometGate, MainCmd, ParaCmd},
    types::{packet::Packet, response::Response},
};

mod ntf_game_time;

#[rustfmt::skip]
pub fn handle(
    Packet {
        main_cmd,
        para_cmd,
        data,
        ..
    }: Packet,
) -> anyhow::Result<Response> {
    assert!(main_cmd == MainCmd::Time || main_cmd == MainCmd::Select);
    let ParaCmd::CometGate(para_cmd) = para_cmd else {
        anyhow::bail!("How did we get here?")
    };

    match para_cmd {
        CometGate::NtfGameTime => todo!(),
        CometGate::RequestUserGameTime => todo!(),
        CometGate::LoginGateVerify => todo!(),
        CometGate::SelectUserInfoList => todo!(),
        CometGate::CreateCharacter => todo!(),
        CometGate::EnterGame => todo!(),

        // NOTE(arjix): When given a client-side param, what should we do?
        _ => unreachable!()
    }
}
