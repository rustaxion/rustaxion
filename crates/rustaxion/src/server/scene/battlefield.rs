use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use tokio::sync::Mutex;

use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{
    ReqBattleFieldBegin, ReqBattleFieldFinish, ReqBattleFieldRankInfo, RetBattleFieldBegin,
    RetBattleFieldFinish, RetBattleFieldInfo, RetBattleFieldRankInfo, SettleData,
};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle_info(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // TODO(arjix): Implement battlefield system.
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseBattleFieldInfo),
        body: RetBattleFieldInfo {
            field_list: vec![],
            player_challenge_count: 3,
            team_challenge_count: 1,
            in_rest_time: 0,
        }
        .encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_rank_info(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqBattleFieldRankInfo::decode(body.as_slice()).context("Failed to decode ReqBattleFieldRankInfo.")?;
    let _ = req;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseBattleFieldRankInfo),
        body: RetBattleFieldRankInfo { list: vec![] }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_begin(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let _req = ReqBattleFieldBegin::decode(body.as_slice()).context("Failed to decode ReqBattleFieldBegin.")?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseBattleFieldBegin),
        body: RetBattleFieldBegin {
            challenge_count: 3,
            settle_data: Some(SettleData { change_list: vec![], update_list: vec![], exp_data: None }),
        }
        .encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_finish(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqBattleFieldFinish::decode(body.as_slice()).context("Failed to decode ReqBattleFieldFinish.")?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseBattleFieldFinish),
        body: RetBattleFieldFinish { field_id: req.field_id, score: 0, rank: 0 }.encode_to_vec(),
    }])
}
