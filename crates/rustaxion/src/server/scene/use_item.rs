use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{entity::*, query::*};
use tokio::sync::Mutex;

use crate::database::entities::{player, player_item, prelude::*};
use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{ReqUseItem, RetUseItem, SettleData, SettleItemData};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqUseItem::decode(body.as_slice()).context("Failed to decode ReqUseItem.")?;
    let player_id = session.lock().await.player_id.context("Not logged in")?;

    let item = PlayerItem::find()
        .filter(player_item::Column::PlayerId.eq(player_id))
        .filter(player_item::Column::ItemId.eq(req.id))
        .one(&db).await?
        .context("Item not in inventory")?;

    anyhow::ensure!(item.count >= 1, "No items remaining");

    let item_type = item.item_type;
    let remaining = item.count - 1;

    // Deduct from inventory
    if remaining == 0 {
        PlayerItem::delete_by_id(item.id).exec(&db).await?;
    } else {
        let mut active: player_item::ActiveModel = item.into();
        active.count = ActiveValue::Set(remaining);
        active.update(&db).await?;
    }

    // Apply item effect
    let mut update_list: Vec<SettleItemData> = vec![];
    let change_list = vec![SettleItemData { r#type: item_type, count: 1, id: req.id }];

    match item_type {
        8 => {
            // Stamina: restore 1 stamina
            let p = Player::find_by_id(player_id).one(&db).await?.context("Player not found")?;
            let new_stamina = (p.current_stamina + 1).min(p.maximum_stamina);
            let mut active: player::ActiveModel = p.clone().into();
            active.current_stamina = ActiveValue::Set(new_stamina);
            active.update(&db).await?;
            update_list.push(SettleItemData { r#type: 8, count: new_stamina, id: 0 });
        }
        10 => {
            // Honour point
            let p = Player::find_by_id(player_id).one(&db).await?.context("Player not found")?;
            let mut active: player::ActiveModel = p.clone().into();
            active.honour_points = ActiveValue::Set(p.honour_points + 100);
            active.update(&db).await?;
            update_list.push(SettleItemData { r#type: 10, count: 100, id: 0 });
        }
        _ => {}
    }

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseUseItem),
        body: RetUseItem {
            id: req.id,
            count: remaining,
            settle_data: SettleData { change_list, update_list, exp_data: None },
            experience: None,
        }.encode_to_vec(),
    }])
}
