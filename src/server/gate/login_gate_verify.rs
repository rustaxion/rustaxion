use std::{sync::Arc, time::SystemTime};

use tokio::sync::Mutex;
use anyhow::Context;
use prost::Message;

use sea_orm::{entity::*, error::*, query::*, DbConn, FromQueryResult};
use entities::account;

use crate::{
    enums::comet::{comet_gate::CometGate, MainCmd, ParaCmd},
    proto::comet_gate::{LoginGateVerify, NotifyGameTime, SelectUserInfo, SelectUserInfoList},
    types::{response::Response, session::SessionData},
};

#[rustfmt::skip]
pub async fn handle(session: &mut SessionData, db: sea_orm::DatabaseConnection, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = LoginGateVerify::decode(buffer.as_slice()).context("Failed to decode LoginGateVerify.")?;
    let mut responses = Vec::<Response>::with_capacity(2);

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let time = NotifyGameTime {
        gametime: u32::try_from(now.as_secs())?,
    };

    responses.push(Response {
        main_cmd: MainCmd::Time,
        para_cmd: ParaCmd::CometGate(CometGate::NotifyGameTime),
        body: time.encode_to_vec()
    });

    let user = account::Entity::find_by_id(req.acc_id).one(&db).await?;
    // TODO(arjix): Make player entity.

    let user_info = SelectUserInfoList {
        user_list: vec![]
    };
    
    responses.push(Response {
        main_cmd: MainCmd::Select,
        para_cmd: ParaCmd::CometGate(CometGate::SelectUserInfoList),
        body: user_info.encode_to_vec()
    });

    Ok(responses)
}
