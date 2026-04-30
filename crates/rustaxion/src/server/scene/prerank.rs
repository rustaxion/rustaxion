use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use tokio::sync::Mutex;

use crate::{
    database::entities::{player, prelude::*},
    types::{response::Response, session::SessionData},
};

use proto::comet_scene::{
    PreRankData, PreRankSingleLevelData, ReqPreRankBegin, ReqPreRankEnd,
    ReqPreRankRankList, RetPreRankBegin, RetPreRankEnd, RetPreRankInfo, RetPreRankRankList,
    SettleData,
};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle_info(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let session = session.lock().await;
    anyhow::ensure!(session.player_id.is_some());

    let player = Player::find_by_id(session.player_id.unwrap()).one(&db).await?;
    anyhow::ensure!(player.is_some());
    let player = player.unwrap();

    let ret = RetPreRankInfo {
        pre_rank: PreRankData { cur_rank: player.pre_rank, list: vec![] },
        pre_rank4_k: PreRankData { cur_rank: player.pre_rank4k, list: vec![] },
        pre_rank6_k: PreRankData { cur_rank: player.pre_rank6k, list: vec![] },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponsePreRankInfo),
        body: ret.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_begin(session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqPreRankBegin::decode(body.as_slice()).context("Failed to decode ReqPreRankBegin.")?;

    let session = session.lock().await;
    anyhow::ensure!(session.player_id.is_some());

    let ret = RetPreRankBegin {
        r#type: req.r#type,
        level_id: req.level_id,
        settle_data: SettleData { change_list: vec![], update_list: vec![], exp_data: None },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponsePreRankBegin),
        body: ret.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_end(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqPreRankEnd::decode(body.as_slice()).context("Failed to decode ReqPreRankEnd.")?;

    let session = session.lock().await;
    anyhow::ensure!(session.player_id.is_some());
    let player_id = session.player_id.unwrap();

    let player = Player::find_by_id(player_id).one(&db).await?;
    anyhow::ensure!(player.is_some());
    let player = player.unwrap();

    let data = req.data;

    // cur_state encodes the rank tier achieved (use it as the rank value to persist)
    let new_rank = data.cur_state;

    // Update pre_rank based on the type (0=default, 1=4K, 2=6K)
    match req.r#type {
        1 => {
            if new_rank > player.pre_rank4k {
                player::ActiveModel {
                    id: ActiveValue::Unchanged(player_id),
                    pre_rank4k: ActiveValue::Set(new_rank),
                    ..Default::default()
                }
                .update(&db)
                .await?;
            }
        }
        2 => {
            if new_rank > player.pre_rank6k {
                player::ActiveModel {
                    id: ActiveValue::Unchanged(player_id),
                    pre_rank6k: ActiveValue::Set(new_rank),
                    ..Default::default()
                }
                .update(&db)
                .await?;
            }
        }
        _ => {
            if new_rank > player.pre_rank {
                player::ActiveModel {
                    id: ActiveValue::Unchanged(player_id),
                    pre_rank: ActiveValue::Set(new_rank),
                    ..Default::default()
                }
                .update(&db)
                .await?;
            }
        }
    }

    let ret = RetPreRankEnd {
        r#type: req.r#type,
        new_rank,
        open_data: Some(data),
        settle_data: Some(SettleData { change_list: vec![], update_list: vec![], exp_data: None }),
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponsePreRankEnd),
        body: ret.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_rank_list(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqPreRankRankList::decode(body.as_slice()).context("Failed to decode ReqPreRankRankList.")?;

    // Return an empty leaderboard — pre-rank leaderboard requires a separate score table.
    let ret = RetPreRankRankList {
        r#type: req.r#type,
        level_id: req.level_id,
        list: vec![],
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponsePreRankRankList),
        body: ret.encode_to_vec(),
    }])
}
