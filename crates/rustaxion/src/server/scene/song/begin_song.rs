use anyhow::Context;
use prost::Message;

use crate::types::{
    response::Response,
    session::{NowPlaying, SessionData},
};

use proto::comet_scene::ReqBeginSong;
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(session: &mut SessionData, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqBeginSong::decode(body.as_slice()).context("Failed to decode ReqBeginSong.")?;
    let now = chrono::Utc::now().timestamp();

    anyhow::ensure!(session.now_playing.is_none());

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
