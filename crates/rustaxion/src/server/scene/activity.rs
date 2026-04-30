use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use tokio::sync::Mutex;

use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{RetActivityFinish, RetActivityInfo, RetActivityBegin, SettleData};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle_info(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // TODO(arjix): Implement activity system.
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseActivityInfo),
        body: RetActivityInfo { list: vec![] }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_begin(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // TODO(arjix): Implement activity system.
    // Returning an error via empty body is not ideal; a proper impl would return ActivityData.
    Ok(vec![])
}

#[rustfmt::skip]
pub async fn handle_finish(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // TODO(arjix): Implement activity system.
    Ok(vec![])
}
