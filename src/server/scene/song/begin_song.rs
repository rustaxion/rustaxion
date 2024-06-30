use anyhow::Context;
use prost::Message;

use crate::{
    enums::comet::{ comet_scene::CometScene, MainCmd, ParaCmd },
    proto::comet_scene::ReqBeginSong,
    types::{ response::Response, session::{ NowPlaying, SessionData } },
};

#[rustfmt::skip]
pub async fn handle(session: &mut SessionData, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqBeginSong::decode(body.as_slice()).context("Failed to decode ReqBeginSong.")?;
    let now = chrono::Utc::now().timestamp();

    // Save the beatmap to the session
    session.now_playing = Some(NowPlaying {
        song_id: req.song_id,
        mode: req.mode,
        difficulty: req.difficulty,
        time: now
    });

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseBeginSong),
        body: vec![]
    }])
}
