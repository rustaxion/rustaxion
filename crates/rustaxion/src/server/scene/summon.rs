use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use tokio::sync::Mutex;

use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{
    ReqSummon, ReqSummonShopBuy, ReqSummonWeekReward, RetSummon, RetSummonInfo,
    RetSummonShopBuy, RetSummonWeekReward, SettleData, SummonWeekInfo,
};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

fn empty_settle() -> SettleData {
    SettleData { change_list: vec![], update_list: vec![], exp_data: None }
}

#[rustfmt::skip]
pub async fn handle_info(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // TODO(arjix): Implement summon/gacha system.
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSummonInfo),
        body: RetSummonInfo {
            item_list: vec![],
            one_normal_price: 100,
            one_real_price: 60,
            five_normal_price: 480,
            five_real_price: 280,
            one_stamina: 60,
            five_stamina: 280,
            lucky_count: 0,
            week_info: SummonWeekInfo { week_count: 0, week_get_list: vec![], reward_list: vec![] },
        }
        .encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_summon(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqSummon::decode(body.as_slice()).context("Failed to decode ReqSummon.")?;
    // TODO(arjix): Implement summon/gacha system.
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSummon),
        body: RetSummon {
            r#type: req.r#type,
            index_list: vec![],
            lucky_count: 0,
            week_count: 0,
            settle_data: empty_settle(),
        }
        .encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_week_reward(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqSummonWeekReward::decode(body.as_slice()).context("Failed to decode ReqSummonWeekReward.")?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSummonWeekReward),
        body: RetSummonWeekReward { count: req.count, settle_data: empty_settle() }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_shop_buy(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqSummonShopBuy::decode(body.as_slice()).context("Failed to decode ReqSummonShopBuy.")?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSummonShopBuy),
        body: RetSummonShopBuy { id: req.id, settle_data: empty_settle() }.encode_to_vec(),
    }])
}
