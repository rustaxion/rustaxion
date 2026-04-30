use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{entity::*, query::*};
use tokio::sync::Mutex;

use crate::database::entities::{mail, player, player_item, prelude::*};
use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{ItemData, MailData, MailList, ReqGetMailReward, RetDelMail, RetGetMailReward, RetMailList, SettleData, SettleItemData};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[derive(serde::Deserialize, serde::Serialize)]
struct MailReward {
    #[serde(rename = "type")]
    item_type: i32,
    count: i32,
    id: i32,
}

fn mail_to_proto(m: &mail::Model) -> MailData {
    let rewards: Vec<ItemData> = serde_json::from_value::<Vec<MailReward>>(m.rewards.clone())
        .unwrap_or_default()
        .into_iter()
        .map(|r| ItemData { r#type: r.item_type, count: r.count, id: r.id })
        .collect();
    MailData {
        mail_id: m.id as u64,
        mail_title: m.title.clone(),
        mail_content: m.content.clone(),
        rewards,
        is_get: m.is_get as i32,
        create_time: m.created_at.timestamp() as i32,
    }
}

#[rustfmt::skip]
pub async fn handle_list(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let player_id = session.lock().await.player_id.context("Not logged in")?;
    let mails = Mail::find().filter(mail::Column::PlayerId.eq(player_id)).all(&db).await?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseMailList),
        body: RetMailList { mail_list: MailList { list: mails.iter().map(mail_to_proto).collect() } }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_get_reward(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqGetMailReward::decode(body.as_slice()).context("Failed to decode ReqGetMailReward.")?;
    let player_id = session.lock().await.player_id.context("Not logged in")?;

    let mail_row = Mail::find_by_id(req.mail_id as i32).one(&db).await?.context("Mail not found")?;
    anyhow::ensure!(mail_row.player_id == player_id, "Mail belongs to another player");
    anyhow::ensure!(!mail_row.is_get, "Rewards already claimed");

    if let Some(expires) = mail_row.expires_at {
        anyhow::ensure!(expires.timestamp() > chrono::Utc::now().timestamp(), "Mail has expired");
    }

    let rewards: Vec<MailReward> = serde_json::from_value(mail_row.rewards.clone()).unwrap_or_default();

    for reward in &rewards {
        match reward.item_type {
            1 => {
                let p = Player::find_by_id(player_id).one(&db).await?.context("Player not found")?;
                let mut active: player::ActiveModel = p.clone().into();
                active.gold = ActiveValue::Set(p.gold + reward.count);
                active.update(&db).await?;
            }
            2 => {
                let p = Player::find_by_id(player_id).one(&db).await?.context("Player not found")?;
                let mut active: player::ActiveModel = p.clone().into();
                active.diamond = ActiveValue::Set(p.diamond + reward.count);
                active.update(&db).await?;
            }
            _ => {
                let existing = PlayerItem::find()
                    .filter(player_item::Column::PlayerId.eq(player_id))
                    .filter(player_item::Column::ItemId.eq(reward.id))
                    .filter(player_item::Column::ItemType.eq(reward.item_type))
                    .one(&db).await?;
                if let Some(item) = existing {
                    let mut active: player_item::ActiveModel = item.clone().into();
                    active.count = ActiveValue::Set(item.count + reward.count);
                    active.update(&db).await?;
                } else {
                    PlayerItem::insert(player_item::ActiveModel {
                        player_id: ActiveValue::Set(player_id),
                        item_id: ActiveValue::Set(reward.id),
                        item_type: ActiveValue::Set(reward.item_type),
                        count: ActiveValue::Set(reward.count),
                        ..Default::default()
                    }).exec(&db).await?;
                }
            }
        }
    }

    mail::ActiveModel {
        id: ActiveValue::Unchanged(mail_row.id),
        is_get: ActiveValue::Set(true),
        ..Default::default()
    }.update(&db).await?;

    let update_list: Vec<SettleItemData> = rewards.iter()
        .map(|r| SettleItemData { r#type: r.item_type, count: r.count, id: r.id })
        .collect();

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseGetMailReward),
        body: RetGetMailReward {
            mail_id: req.mail_id,
            settle_data: Some(SettleData { change_list: vec![], update_list, exp_data: None }),
        }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_delete(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let player_id = session.lock().await.player_id.context("Not logged in")?;
    Mail::delete_many()
        .filter(mail::Column::PlayerId.eq(player_id))
        .filter(mail::Column::IsGet.eq(true))
        .exec(&db).await?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseDeleteMail),
        body: RetDelMail {}.encode_to_vec(),
    }])
}
