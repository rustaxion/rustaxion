use std::sync::Arc;

use crate::types::{response::Response, session::SessionData};

use proto::{
    enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd},
    packet::Packet,
};
use tokio::sync::Mutex;

mod announcement;
mod bind_account;
mod bili_login;
mod find_password;
mod game_version;
mod login_account;
mod quick_login;
mod quick_token;
mod reg_account;
mod third_party_login;

#[rustfmt::skip]
pub async fn handle(
    session: Arc<Mutex<SessionData>>,
    db: sea_orm::DatabaseConnection,
    Packet {
        main_cmd,
        para_cmd,
        data,
        ..
    }: Packet,
) -> anyhow::Result<Vec<Response>> {
    assert_eq!(main_cmd, MainCmd::Login);
    let ParaCmd::CometLogin(para_cmd) = para_cmd else {
        anyhow::bail!("How did we get here?")
    };

    match para_cmd {
        CometLogin::RequestRegAccount => reg_account::handle(session, db, data).await,
        CometLogin::RequestLoginAccount => login_account::handle(session, db, data).await,
        CometLogin::RequestFindPassword => find_password::handle(session, db, data).await,
        CometLogin::RequestQuickToken => quick_token::handle(session, db, data).await,
        CometLogin::RequestQuickLogin => quick_login::handle(session, db, data).await,
        CometLogin::RequestThirdLogin => third_party_login::handle(session, db, data).await,
        CometLogin::RequestBindAccount => bind_account::handle(session, db, data).await,
        CometLogin::RequestAnnouncement => announcement::handle(session, db, data).await,
        CometLogin::RequestGameVersion => game_version::handle(session, db, data).await,
        CometLogin::RequestBiliLogin => bili_login::handle(session, db, data).await,

        // NOTE(arjix): When given a client-side param, what should we do?
        _ => unreachable!()
    }
}
