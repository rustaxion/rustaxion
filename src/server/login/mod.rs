use crate::{
    enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd},
    types::{packet::Packet, response::Response, session::SessionData},
};

mod game_version;
mod third_party_login;

#[rustfmt::skip]
pub fn handle(
    session: &mut SessionData,
    Packet {
        main_cmd,
        para_cmd,
        data,
        ..
    }: Packet,
) -> anyhow::Result<Response> {
    assert_eq!(main_cmd, MainCmd::Login);
    let ParaCmd::CometLogin(para_cmd) = para_cmd else {
        anyhow::bail!("How did we get here?")
    };

    match para_cmd {
        CometLogin::RequestRegAccount => todo!(),
        CometLogin::RequestLoginAccount => todo!(),
        CometLogin::RequestFindPassword => todo!(),
        CometLogin::RequestQuickToken => todo!(),
        CometLogin::RequestQuickLogin => todo!(),
        CometLogin::RequestThirdLogin => third_party_login::handle(session, data),
        CometLogin::RequestBindAccount => todo!(),
        CometLogin::RequestAnnouncement => todo!(),
        CometLogin::RequestGameVersion => game_version::handle(session, data),
        CometLogin::RequestBiliLogin => todo!(),

        // NOTE(arjix): When given a client-side param, what should we do?
        _ => unreachable!()
    }
}
