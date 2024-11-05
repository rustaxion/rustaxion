use anyhow::Context;
use prost::Message;

use crate::database::entities::sea_orm_active_enums::{Country, Language};
use crate::database::entities::{player, player_character, prelude::*};
use crate::proto;
use sea_orm::entity::*;

use crate::database::helpers::get_character_full_data;
use crate::enums::comet::comet_gate::CometGate;
use crate::proto::comet_gate::{SelectUserInfo, SelectUserInfoList};
use crate::proto::comet_scene::NotifyCharacterFullData;
use crate::{
    enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd},
    proto::{
        comet_gate::CreateCharacter,
        comet_scene::{CharData, CharacterFullData, CharacterList},
    },
    types::{response::Response, session::SessionData},
};

pub async fn handle(
    session: &mut SessionData,
    db: sea_orm::DatabaseConnection,
    buffer: Vec<u8>,
) -> anyhow::Result<Vec<Response>> {
    anyhow::ensure!(session.account_id.is_some());

    let req =
        CreateCharacter::decode(buffer.as_slice()).context("Failed to decode CreateCharacter.")?;
    let mut responses = Vec::<Response>::with_capacity(2);

    let player = Player::insert(player::ActiveModel {
        account_id: Set(session.account_id.unwrap() as i32),
        head_id: Set(req.select_char_id as i32),
        title_id: Set(10001),
        name: Set(req.name),
        language: Set(Language::from_proto(
            proto::comet_login::LanguageType::try_from(req.language as i32)?,
        )),
        country: Set(Country::from_proto(proto::comet_scene::Country::try_from(
            req.country as i32,
        )?)),
        selected_character_id: Set(req.select_char_id as i32),
        selected_theme_id: Set(1),
        ..Default::default()
    })
    .exec(&db)
    .await?;

    PlayerCharacter::insert(player_character::ActiveModel {
        player_id: Set(player.last_insert_id),
        character_id: Set(req.select_char_id as i32),
        level: Set(1),
        experience: Set(0),
        play_count: Set(0),
    })
    .exec(&db)
    .await?;

    let full_data: CharacterFullData = get_character_full_data(player.last_insert_id, &db).await?;

    responses.push(Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::NotifyCharacterFullData),
        body: (NotifyCharacterFullData {
            data: full_data.clone(),
        })
        .encode_to_vec(),
    });

    let players = Player::find_by_id(full_data.base_info.acc_id)
        .all(&db)
        .await?;

    let user_info = SelectUserInfoList {
        user_list: players
            .iter()
            .map(|p| SelectUserInfo {
                char_id: p.account_id as i64,
                acc_states: 0,
            })
            .collect::<Vec<_>>(),
    };

    responses.push(Response {
        main_cmd: MainCmd::Select,
        para_cmd: ParaCmd::CometGate(CometGate::SelectUserInfoList),
        body: user_info.encode_to_vec(),
    });

    Ok(responses)
}
