use std::time::SystemTime;

use tokio::sync::Mutex;
use anyhow::Context;
use prost::Message;

use sea_orm::{ entity::*, error::*, query::*, DbConn, FromQueryResult };
use crate::database::entities::{ player, prelude::* };

use crate::database::helpers::get_character_full_data;
use crate::{
    enums::comet::{ comet_scene::CometScene, MainCmd, ParaCmd },
    proto::{
        comet_gate::CreateCharacter,
        comet_scene::{ AnnouncementData, CharData, CharacterFullData, CharacterList },
    },
    types::{ response::Response, session::SessionData },
};

pub async fn handle(
    session: &mut SessionData,
    db: sea_orm::DatabaseConnection,
    buffer: Vec<u8>
) -> anyhow::Result<Vec<Response>> {
    anyhow::ensure!(session.account_id.is_some());

    let req = CreateCharacter::decode(buffer.as_slice()).context(
        "Failed to decode CreateCharacter."
    )?;

    Player::insert(player::ActiveModel {
        account_id: ActiveValue::Set(session.account_id.unwrap()),
        character_id: ActiveValue::Set((session.account_id.unwrap() as i64) + 40000000000),
        head_id: ActiveValue::Set(req.select_char_id as i32),
        title_id: ActiveValue::Set(10001),
        name: ActiveValue::Set(req.name),
        language: ActiveValue::Set(req.language as i32),
        country: ActiveValue::Set(req.country as i32),
        selected_character_id: ActiveValue::Set(req.select_char_id as i32),
        selected_theme_id: ActiveValue::Set(1),
        ..Default::default()
    }).exec(&db).await?;

    let mut full_data: CharacterFullData = get_character_full_data(
        session.account_id.unwrap(),
        &db
    ).await?;

    full_data.char_list = CharacterList {
        list: vec![CharData {
            char_id: req.select_char_id as i32,
            level: 1,
            exp: 0,
            play_count: 0,
        }],
    };

    Ok(
        vec![Response {
            main_cmd: MainCmd::Game,
            para_cmd: ParaCmd::CometScene(CometScene::NotifyCharacterFullData),
            body: full_data.encode_to_vec(),
        }]
    )
}
