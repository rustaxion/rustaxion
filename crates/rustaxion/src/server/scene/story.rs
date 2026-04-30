use std::sync::Arc;

use prost::Message;
use tokio::sync::Mutex;

use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{RetStoryFinish, RetStoryInfo, SettleData, StoryData};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle_info(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // TODO(arjix): Implement story mode system.
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseStoryInfo),
        body: RetStoryInfo {
            cur_normal_chapter_id: 1,
            cur_normal_level_id: 1,
            cur_tutorial_chapter_id: 1,
            cur_tutorial_level_id: 1,
            list: vec![],
            special_list: vec![],
        }
        .encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_finish(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // TODO(arjix): Implement story mode system.
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseStoryFinish),
        body: RetStoryFinish {
            data: StoryData { chapter_id: 1, level_id: 1, max_score: 0, max_combo: 0, finish_level: 0, mission_list: vec![], cur_rank: 0 },
            settle_data: Some(SettleData { change_list: vec![], update_list: vec![], exp_data: None }),
            cur_tutorial_chapter_id: Some(1),
            cur_tutorial_level_id: Some(1),
            cur_normal_chapter_id: Some(1),
            cur_normal_level_id: Some(1),
            cur_special_chapter_id: Some(0),
            cur_special_level_id: Some(0),
        }
        .encode_to_vec(),
    }])
}
