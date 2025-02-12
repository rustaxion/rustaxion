use anyhow::Context;
use prost::Message;

use crate::types::{response::Response, session::SessionData};

use proto::comet_login::{ReqGameVersion, RetGameVersion};
use proto::enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(_session: &mut SessionData, _db: sea_orm::DatabaseConnection, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let _req = ReqGameVersion::decode(buffer.as_slice()).context("Failed to decode ReqGameVersion.")?;
    let ret = RetGameVersion {
        version: "0.1.0".to_string(),
        server_state: 2,
        announcement_title: "".to_string(),
        announcement_content: "".to_string(),
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnGameVersion),
        body: ret.encode_to_vec()
    }])
}
