use anyhow::Context;
use prost::Message;

use crate::{
    enums::comet::{ comet_scene::CometScene, MainCmd, ParaCmd },
    proto::comet_scene::{ ReqFinishSong, RetFinishSong },
    types::{ response::Response, session::SessionData },
};

#[rustfmt::skip]
pub async fn handle(session: &mut SessionData, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let now = chrono::Utc::now().timestamp();
    let req = ReqFinishSong::decode(body.as_slice()).context("Failed to decode ReqFinishSong.")?;
    anyhow::ensure!(session.now_playing.is_some());
    let song_data = req.data;

    let now_playing = session.now_playing.as_ref().unwrap();

    eprintln!("{:?}", now_playing);
    anyhow::ensure!(now_playing.song_id == song_data.song_id);
    anyhow::ensure!(now_playing.mode == song_data.mode);

    eprintln!("Completed beatmap {} in {} seconds", song_data.song_id, now_playing.time - now);
    session.now_playing = None;

    // TODO(arjix): Once beatmap duration is known server-side,
    // ensure the completion-time matches the duration of the beatmap.

    // TODO(arjix): Port the logic from the emulator's implementation
    // https://github.com/Invaxion-Server-Emulator/invaxion-server-emulator/blob/master/Server/Emulator/Handlers/GateHandlers/Beatmaps.cs#L262-L455

    // let ret = RetFinishSong {
    //     song_info: todo!(),
    //     settle_data: todo!(),
    //     new_rank: todo!()
    // };

    Ok(vec![
        // Response {
        //     main_cmd: MainCmd::Game,
        //     para_cmd: ParaCmd::CometScene(CometScene::ResponseFinishSong),
        //     body: ret.encode_to_vec()
        // }
    ])
}
