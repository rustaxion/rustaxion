use anyhow::Context;
use prost::Message;

use crate::{
    enums::comet::{ comet_scene::CometScene, MainCmd, ParaCmd },
    proto::comet_scene::{ ReqFinishSong, RetFinishSong },
    types::{ response::Response, session::SessionData },
};

#[rustfmt::skip]
pub async fn handle(_session: &mut SessionData, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqFinishSong::decode(body.as_slice()).context("Failed to decode ReqFinishSong.")?;

    // TODO(arjix): Port the logic from the emulator's implementation
    // https://github.com/Invaxion-Server-Emulator/invaxion-server-emulator/blob/master/Server/Emulator/Handlers/GateHandlers/Beatmaps.cs#L262-L455

    let ret = RetFinishSong {
        song_info: todo!(),
        settle_data: todo!(),
        new_rank: todo!()
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseFinishSong),
        body: ret.encode_to_vec()
    }])
}
