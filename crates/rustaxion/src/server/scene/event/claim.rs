/// All event claim handlers share the same response shape: Ret_Event_GetCommon { index, settle_data }.
/// Since event data is not yet persisted in the DB, these return empty SettleData.
use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use tokio::sync::Mutex;

use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{
    ReqEventBili, ReqEventFriend, ReqEventLevelGift, ReqEventLogin, ReqEventNewCharLogin,
    ReqEventNewCharRelease, ReqEventNewPlayer, ReqEventNewThemeLogin, ReqEventNewThemeRelease,
    ReqEventRecharge, ReqEventWeekCheckin, RetEventGetCommon, SettleData,
};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

// All event claim responses use the same ResponseEventGetCommon variant.
fn empty_ret(index: i32) -> Vec<Response> {
    vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseEventGetCommon),
        body: RetEventGetCommon {
            index,
            settle_data: SettleData { change_list: vec![], update_list: vec![], exp_data: None },
        }
        .encode_to_vec(),
    }]
}

pub async fn handle_level_gift(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqEventLevelGift::decode(body.as_slice()).context("Failed to decode ReqEventLevelGift.")?;
    Ok(empty_ret(req.level))
}

pub async fn handle_stamina(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(empty_ret(0))
}

pub async fn handle_new_player(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqEventNewPlayer::decode(body.as_slice()).context("Failed to decode ReqEventNewPlayer.")?;
    Ok(empty_ret(req.day))
}

pub async fn handle_week_checkin(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqEventWeekCheckin::decode(body.as_slice()).context("Failed to decode ReqEventWeekCheckin.")?;
    Ok(empty_ret(req.day))
}

pub async fn handle_recharge(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqEventRecharge::decode(body.as_slice()).context("Failed to decode ReqEventRecharge.")?;
    Ok(empty_ret(req.index))
}

pub async fn handle_login(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqEventLogin::decode(body.as_slice()).context("Failed to decode ReqEventLogin.")?;
    Ok(empty_ret(req.index))
}

pub async fn handle_new_char_login(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqEventNewCharLogin::decode(body.as_slice()).context("Failed to decode ReqEventNewCharLogin.")?;
    Ok(empty_ret(req.index))
}

pub async fn handle_new_theme_login(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqEventNewThemeLogin::decode(body.as_slice()).context("Failed to decode ReqEventNewThemeLogin.")?;
    Ok(empty_ret(req.index))
}

pub async fn handle_new_char_release(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqEventNewCharRelease::decode(body.as_slice()).context("Failed to decode ReqEventNewCharRelease.")?;
    Ok(empty_ret(req.index))
}

pub async fn handle_new_theme_release(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqEventNewThemeRelease::decode(body.as_slice()).context("Failed to decode ReqEventNewThemeRelease.")?;
    Ok(empty_ret(req.index))
}

pub async fn handle_friend(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqEventFriend::decode(body.as_slice()).context("Failed to decode ReqEventFriend.")?;
    Ok(empty_ret(req.index))
}

pub async fn handle_bili(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqEventBili::decode(body.as_slice()).context("Failed to decode ReqEventBili.")?;
    Ok(empty_ret(req.index))
}
