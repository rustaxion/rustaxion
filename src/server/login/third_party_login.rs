use anyhow::Context;
use prost::Message;

use crate::{
    enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd},
    proto::comet_login::{
        GatewayServerData, ReqGameVersion, ReqThirdLogin, RetGameVersion, RetThirdLogin,
    },
    types::{response::Response, session::SessionData},
};

#[rustfmt::skip]
pub fn handle(session: &mut SessionData, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqThirdLogin::decode(buffer.as_slice()).context("Failed to decode ReqThirdLogin.")?;
    let token = format!("{:x}", md5::compute(req.open_id + "6031"));

    let ret = RetThirdLogin {
        data: GatewayServerData {
            gate_ip: "127.0.0.1".to_string(),
            gate_port: 6969,
            acc_id: 10,
            token,
        }
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnThirdLogin),
        body: ret.encode_to_vec()
    }])
}
