use std::sync::Arc;

use prost::Message;
use tokio::sync::Mutex;

use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{RetArcadeFinish, RetArcadeInfo, SettleData};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle_info(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // TODO(arjix): Implement arcade system.
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseArcadeInfo),
        body: RetArcadeInfo { stage_list: vec![] }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_finish(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // TODO(arjix): Implement arcade system.
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseArcadeFinish),
        body: RetArcadeFinish {
            settle_data: Some(SettleData { change_list: vec![], update_list: vec![], exp_data: None }),
        }
        .encode_to_vec(),
    }])
}
