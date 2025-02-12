use prost::Message;
use sea_orm::{entity::*, query::*};

use crate::{
    database::entities::{daily_login, prelude::*},
    types::{response::Response, session::SessionData},
};

use proto::comet_scene::{
    GetStaminaData, ItemData, LevelGiftData, NewPlayerData, RetEventInfo, WeekCheckinData,
    WeekCheckinRewardData,
};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(session: &mut SessionData, db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // let req = ReqEventInfo::decode(body.as_slice()).context("Failed to decode ReqEventInfo.")?;
    anyhow::ensure!(session.player_id.is_some());

    let daily = DailyLogin::find().filter(
        daily_login::Column::PlayerId.eq(session.player_id.unwrap() as i32)
    ).one(&db).await?.ok_or(anyhow::anyhow!("Expected a daily login to already exist."))?;

    // TODO(arjix): Once a web-ui is made, make this more customizable.
    // TODO(arjix): I should probably return a reward message for the daily login??

    let ret = RetEventInfo {
        level_gift: LevelGiftData { get_list: vec![5, 2] },
        get_stamina: GetStaminaData { is_get: 0 },
        new_player: NewPlayerData {
            login_day: daily.login_counter,
            get_list: vec![]
        },
        week_checkin: WeekCheckinData {
            login_day: daily.login_counter,
            get_list: vec![],
            reward_list: vec![
                WeekCheckinRewardData { day: 1, reward: ItemData { id: 0, r#type: 1, count: 10 } },
                WeekCheckinRewardData { day: 2, reward: ItemData { id: 90002, r#type: 3, count: 1 } },
                WeekCheckinRewardData { day: 3, reward: ItemData { id: 90001, r#type: 3, count: 1 } },
                WeekCheckinRewardData { day: 4, reward: ItemData { id: 0, r#type: 1, count: 10 } },
                WeekCheckinRewardData { day: 5, reward: ItemData { id: 90001, r#type: 3, count: 1 } },
                WeekCheckinRewardData { day: 6, reward: ItemData { id: 90002, r#type: 3, count: 1 } },
                WeekCheckinRewardData { day: 7, reward: ItemData { id: 90005, r#type: 3, count: 1 } },
            ],
        },
        recharge: None,
        login: None,
        new_char_login: None,
        new_theme_login: None,
        new_char_release: None,
        new_theme_release: None,
        friend: None,
        bili: None,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseEventInfo),
        body: ret.encode_to_vec()
    }])
}
