use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use tokio::sync::Mutex;

use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{
    ReqPvpCurState, RetPvpBeginMatching, RetPvpCurState, RetPvpEndMatching, RetPvpMatchConfim,
};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

// PVP requires a shared matchmaking state (not per-connection), so all handlers are stubs.

#[rustfmt::skip]
pub async fn handle_begin_matching(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponsePVPBeginMatching),
        body: RetPvpBeginMatching {}.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_end_matching(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponsePVPEndMatching),
        body: RetPvpEndMatching {}.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_match_confirm(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponsePVPMatchConfirm),
        body: RetPvpMatchConfim {}.encode_to_vec(),
    }])
}

// Finish loading, sync score, use skill, and finish game are broadcast-only — no direct Ret_.
#[rustfmt::skip]
pub async fn handle_finish_loading(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![])
}

#[rustfmt::skip]
pub async fn handle_sync_score(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![])
}

#[rustfmt::skip]
pub async fn handle_use_skill(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![])
}

#[rustfmt::skip]
pub async fn handle_finish_game(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![])
}

#[rustfmt::skip]
pub async fn handle_current_state(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let _req = ReqPvpCurState::decode(body.as_slice()).context("Failed to decode ReqPvpCurState.")?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponsePVPCurrentState),
        body: RetPvpCurState { state: 0 }.encode_to_vec(),
    }])
}
