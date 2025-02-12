use anyhow::Context;
use prost::Message;
use sea_orm::{entity::*, QueryFilter};

use crate::database::entities::sea_orm_active_enums::{Country, Language};
use crate::database::entities::{player, player_character, player_theme, prelude::*};

use crate::database::helpers::get_player_full_data;
use crate::types::{response::Response, session::SessionData};

use proto::enums::comet::{comet_gate::CometGate, comet_scene::CometScene, MainCmd, ParaCmd};

use proto::comet_gate::{CreateCharacter, SelectUserInfo, SelectUserInfoList};
use proto::comet_scene::{CharacterFullData, NotifyCharacterFullData};

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

    PlayerTheme::insert(player_theme::ActiveModel {
        player_id: Set(player.last_insert_id),
        theme_id: Set(1),
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

    // character full data
    {
        let data: CharacterFullData = get_player_full_data(player.last_insert_id, &db).await?;

        responses.push(Response {
            main_cmd: MainCmd::Game,
            para_cmd: ParaCmd::CometScene(CometScene::NotifyCharacterFullData),
            body: (NotifyCharacterFullData { data }).encode_to_vec(),
        });
    }

    // all characters for account
    {
        let players = Player::find()
            .filter(player::Column::AccountId.eq(session.account_id.unwrap()))
            .all(&db)
            .await?;

        let user_info = SelectUserInfoList {
            user_list: players
                .iter()
                .map(|p| SelectUserInfo {
                    char_id: p.id as i64,
                    acc_states: 0,
                })
                .collect::<Vec<_>>(),
        };

        responses.push(Response {
            main_cmd: MainCmd::Select,
            para_cmd: ParaCmd::CometGate(CometGate::SelectUserInfoList),
            body: user_info.encode_to_vec(),
        });
    }

    Ok(responses)
}
