use std::time::SystemTime;

use tokio::sync::Mutex;
use anyhow::Context;
use prost::Message;

use sea_orm::{ entity::*, error::*, query::*, DbConn, FromQueryResult };
use crate::database::entities::prelude::*;

use crate::database::helpers::get_character_full_data;
use crate::enums::comet::comet_scene::CometScene;
use crate::proto::comet_scene::{ CharData, CharacterFullData, CharacterList };
use crate::{
    enums::comet::{ comet_gate::CometGate, MainCmd, ParaCmd },
    proto::comet_gate::EnterGame,
    types::{ response::Response, session::SessionData },
};

#[rustfmt::skip]
pub async fn handle(session: &mut SessionData, db: sea_orm::DatabaseConnection, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = EnterGame::decode(buffer.as_slice()).context("Failed to decode EnterGame.")?;
    let mut full_data: CharacterFullData = get_character_full_data(
        session.account_id.unwrap(),
        &db
    ).await?;

    full_data.char_list = CharacterList {
        list: vec![CharData {
            char_id: 30040,
            level: 1,
            exp: 0,
            play_count: 0,
        }],
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::NotifyCharacterFullData),
        body: full_data.encode_to_vec(),
    }])
}
