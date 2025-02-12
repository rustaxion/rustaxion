use prost::Message;

use crate::types::{response::Response, session::SessionData};

use proto::comet_gate::NotifyGameTime;
use proto::enums::comet::{comet_gate::CometGate, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(_session: &mut SessionData, _db: sea_orm::DatabaseConnection, _buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Time,
        para_cmd: ParaCmd::CometGate(CometGate::ResponseUserGameTime),
        body: NotifyGameTime { gametime: chrono::Utc::now().timestamp() as i32 }.encode_to_vec()
    }])
}
