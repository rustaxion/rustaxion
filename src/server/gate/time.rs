use prost::Message;

use crate::{
    enums::comet::{ comet_gate::CometGate, MainCmd, ParaCmd },
    proto::comet_gate::{ NotifyGameTime, RetUserGameTime },
    types::{ response::Response, session::SessionData },
};

#[rustfmt::skip]
pub async fn handle(_session: &mut SessionData, _db: sea_orm::DatabaseConnection, _buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Time,
        para_cmd: ParaCmd::CometGate(CometGate::ResponseUserGameTime),
        body: NotifyGameTime { gametime: chrono::Utc::now().timestamp() as i32 }.encode_to_vec()
    }])
}
