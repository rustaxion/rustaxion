use anyhow::Context;
use prost::Message;

use chrono::prelude::*;
use sea_orm::{entity::*, query::*};

use crate::database::entities::{daily_login, prelude::*};
use crate::database::helpers::get_character_full_data;
use crate::enums::comet::comet_scene::CometScene;
use crate::proto::comet_scene::{
    CharData, CharacterFullData, CharacterList, NotifyCharacterFullData,
};
use crate::{
    enums::comet::{MainCmd, ParaCmd},
    proto::comet_gate::EnterGame,
    types::{response::Response, session::SessionData},
};

#[rustfmt::skip]
pub async fn handle(session: &mut SessionData, db: sea_orm::DatabaseConnection, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = EnterGame::decode(buffer.as_slice()).context("Failed to decode EnterGame.")?;
    session.player_id = Some(req.char_id as i32);

    //=// daily login
    let now = chrono::Utc::now();
    let daily = DailyLogin::find().filter(daily_login::Column::PlayerId.eq(req.char_id)).one(&db).await?;
    match daily {
        Some(daily) => {
            let date_time = DateTime::from_timestamp(daily.last_day_login.to_utc().timestamp(), 0)
                .ok_or(anyhow::anyhow!("Failed to parse unix timestamp."))?;

            let today = now.day() as i32;
            let other_day = date_time.day() as i32;

            let is_new_day = today - other_day == 1;
            let broke_streak = today - other_day > 1;

            let mut daily = daily;
            if is_new_day {
                // player is on a streak
                daily.login_counter += 1;
                daily.login_streak += 1;
            } else if broke_streak {
                // player is either on the first day, or broke the streak
                daily.login_counter = 1;
                daily.login_streak = 1;
            } else {
                // player has re-entered the game on the same day
            }

            if is_new_day || broke_streak {
                daily.last_day_login = now.fixed_offset();
            }

            daily_login::ActiveModel {
                id: Unchanged(daily.id),
                player_id: Unchanged(daily.player_id),
                login_counter: Set(daily.login_counter),
                login_streak: Set(daily.login_streak),
                last_day_login: Set(daily.last_day_login)
            }.update(&db).await?;
        },
        None => {
            daily_login::ActiveModel {
                player_id: Unchanged(session.player_id.unwrap()),
                login_counter: Set(1),
                login_streak: Set(1),
                last_day_login: Set(now.fixed_offset()),
                ..Default::default()
            }.insert(&db).await?;
        }
    }
    //=// end daily login

    let full_data: CharacterFullData = get_character_full_data(session.player_id.unwrap(), &db).await?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::NotifyCharacterFullData),
        body: NotifyCharacterFullData { data: full_data }.encode_to_vec(),
    }])
}
