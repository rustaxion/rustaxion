use std::sync::Arc;

use prost::Message;
use tokio::sync::Mutex;

use crate::types::{response::Response, session::SessionData};

use proto::comet_login::RetAnnouncement;
use proto::enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // TODO(arjix): Load announcement from the database or a config file.
    let ret = RetAnnouncement {
        title: "Welcome to Rustaxion".to_string(),
        content: "A reverse-engineered INVAXION server. Have fun!".to_string(),
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnAnnouncement),
        body: ret.encode_to_vec(),
    }])
}
