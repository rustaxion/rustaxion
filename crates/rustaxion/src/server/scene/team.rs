use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use tokio::sync::Mutex;

use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{
    ReqTeamApply, ReqTeamBuyItem, ReqTeamConfimUploadSong, ReqTeamCreate, ReqTeamDealApply,
    ReqTeamDeclaration, ReqTeamKick, ReqTeamPosition, ReqTeamSearch, ReqTeamUploadSong,
    RetTeamApply, RetTeamApplyList, RetTeamBuyItem, RetTeamConfimUploadSong, RetTeamCreate,
    RetTeamDealApply, RetTeamDeclaration, RetTeamExit, RetTeamInfo, RetTeamKick, RetTeamList,
    RetTeamLogs, RetTeamPosition, RetTeamSearch, RetTeamUploadSong, TeamShopData, TeamUploadSongData,
};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

// All team handlers are stubs — team infrastructure (DB tables) is not yet implemented.

#[rustfmt::skip]
pub async fn handle_create(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamCreate),
        body: RetTeamCreate { is_success: 0, left_time: None, info: None }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_search(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamSearch),
        body: RetTeamSearch { is_find: 0, info: None }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_list(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamList),
        body: RetTeamList { list: vec![], apply_list: vec![] }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_apply(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamApply),
        body: RetTeamApply { is_success: 0, left_time: None }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_declaration(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamDeclaration),
        body: RetTeamDeclaration {}.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_info(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // Client expects TeamInfoData; return an error/empty response since player is not in a team.
    Ok(vec![])
}

#[rustfmt::skip]
pub async fn handle_position(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamPosition),
        body: RetTeamPosition {}.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_apply_list(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamApplyList),
        body: RetTeamApplyList { list: vec![] }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_deal_apply(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqTeamDealApply::decode(body.as_slice()).context("Failed to decode ReqTeamDealApply.")?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamDealApply),
        body: RetTeamDealApply { apply_char_id: req.apply_char_id, new_member: None, member_count: None }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_kick(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqTeamKick::decode(body.as_slice()).context("Failed to decode ReqTeamKick.")?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamKick),
        body: RetTeamKick { member_id: req.member_id, member_count: 0 }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_exit(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamExit),
        body: RetTeamExit {}.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_logs(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamLogs),
        body: RetTeamLogs { log_list: vec![] }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_upload_song(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamUploadSong),
        body: RetTeamUploadSong { upload_song_count: 0 }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_confirm_upload_song(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamConfirmUploadSong),
        body: RetTeamConfimUploadSong {}.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_buy_item(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTeamBuyItem),
        body: RetTeamBuyItem {
            shop: TeamShopData { honour_point: 0, shop_list: vec![] },
        }
        .encode_to_vec(),
    }])
}
