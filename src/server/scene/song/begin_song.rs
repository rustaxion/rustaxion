use anyhow::Context;
use prost::Message;

use crate::{
    enums::comet::{ comet_scene::CometScene, MainCmd, ParaCmd },
    proto::comet_scene::ReqBeginSong,
    types::{ response::Response, session::SessionData },
};

#[rustfmt::skip]
pub async fn handle(_session: &mut SessionData, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let _req = ReqBeginSong::decode(body.as_slice()).context("Failed to decode ReqBeginSong.")?;

    // TODO(arjix): Maybe store this in a table, or the session data?

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseBeginSong),
        body: vec![]
    }])
}
