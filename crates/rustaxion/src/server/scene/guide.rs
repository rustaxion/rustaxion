use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{ActiveModelTrait, ActiveValue};
use tokio::sync::Mutex;

use crate::{
    database::entities::{player, prelude::*},
    types::{response::Response, session::SessionData},
};

use proto::comet_scene::{ReqGuide, RetGuideClear, RetGuide};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle_guide(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqGuide::decode(body.as_slice()).context("Failed to decode ReqGuide.")?;

    let session = session.lock().await;
    anyhow::ensure!(session.player_id.is_some());

    player::ActiveModel {
        id: ActiveValue::Unchanged(session.player_id.unwrap()),
        guide_step: ActiveValue::Set(req.step),
        ..Default::default()
    }
    .update(&db)
    .await?;

    let ret = RetGuide { step: req.step };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseGuide),
        body: ret.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_guide_clear(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let session = session.lock().await;
    anyhow::ensure!(session.player_id.is_some());

    player::ActiveModel {
        id: ActiveValue::Unchanged(session.player_id.unwrap()),
        guide_step: ActiveValue::Set(i32::MAX),
        ..Default::default()
    }
    .update(&db)
    .await?;

    let ret = RetGuideClear {};

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseGuideClear),
        body: ret.encode_to_vec(),
    }])
}
