use std::time::SystemTime;

use tokio::sync::Mutex;
use anyhow::Context;
use prost::Message;

use sea_orm::{ entity::*, error::*, query::*, DbConn, FromQueryResult };
use crate::database::entities::prelude::*;

use crate::{
    enums::comet::{ comet_gate::CometGate, MainCmd, ParaCmd },
    proto::comet_gate::{ LoginGateVerify, NotifyGameTime, SelectUserInfo, SelectUserInfoList },
    types::{ response::Response, session::SessionData },
};

#[rustfmt::skip]
pub async fn handle(session: &mut SessionData, db: sea_orm::DatabaseConnection, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = LoginGateVerify::decode(buffer.as_slice()).context("Failed to decode LoginGateVerify.")?;
    let mut responses = Vec::<Response>::with_capacity(2);

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    responses.push(Response {
        main_cmd: MainCmd::Time,
        para_cmd: ParaCmd::CometGate(CometGate::NotifyGameTime),
        body: NotifyGameTime { gametime: u32::try_from(now.as_secs())? }.encode_to_vec()
    });

    let user = Account::find_by_id(req.acc_id as i32).one(&db).await?;
    session.account_id = user.clone().map(|x| x.id);
    anyhow::ensure!(user.is_some());

    let user = user.unwrap();
    let players = Player::find_by_id(user.id).all(&db).await?;

    let user_info = SelectUserInfoList {
        user_list: players
            .iter()
            .map(|p| SelectUserInfo {
                char_id: p.account_id as u64,
                acc_states: 0
            })
            .collect::<Vec<_>>()
    };
    
    responses.push(Response {
        main_cmd: MainCmd::Select,
        para_cmd: ParaCmd::CometGate(CometGate::SelectUserInfoList),
        body: user_info.encode_to_vec()
    });

    Ok(responses)
}
