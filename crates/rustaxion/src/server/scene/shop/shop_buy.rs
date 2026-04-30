use std::sync::Arc;

use anyhow::Context;
use chrono::Utc;
use prost::Message;
use sea_orm::{entity::*, query::*};
use tokio::sync::Mutex;

use crate::database::entities::{
    player, player_beatmap, player_character, player_theme, prelude::*,
    sea_orm_active_enums::ShopItemType,
};
use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{ReqShopBuy, RetShopBuy, SettleData, SettleItemData};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqShopBuy::decode(body.as_slice()).context("Failed to decode ReqShopBuy.")?;
    let player_id = session.lock().await.player_id.context("Not logged in")?;

    let shop_item = ShopItem::find_by_id(req.item_id).one(&db).await?.context("Shop item not found")?;

    let now = Utc::now().timestamp();
    let in_discount = shop_item.discount_begin_time.timestamp() <= now
        && now <= shop_item.discount_end_time.timestamp();
    let price = if in_discount { shop_item.discount_price } else { shop_item.normal_price };
    let cost_type = shop_item.cost_type;

    let player_row = Player::find_by_id(player_id).one(&db).await?.context("Player not found")?;

    match cost_type {
        1 => anyhow::ensure!(player_row.gold >= price, "Not enough gold"),
        2 => anyhow::ensure!(player_row.diamond >= price, "Not enough diamond"),
        _ => {}
    }

    // Deduct currency
    let mut player_active: player::ActiveModel = player_row.clone().into();
    match cost_type {
        1 => { player_active.gold = ActiveValue::Set(player_row.gold - price); }
        2 => { player_active.diamond = ActiveValue::Set(player_row.diamond - price); }
        _ => {}
    }
    player_active.update(&db).await?;

    // Grant item and resolve settle type
    let item_settle_type = match shop_item.item_type {
        ShopItemType::Character => {
            let exists = PlayerCharacter::find()
                .filter(player_character::Column::PlayerId.eq(player_id))
                .filter(player_character::Column::CharacterId.eq(req.item_id))
                .one(&db).await?.is_some();
            if !exists {
                PlayerCharacter::insert(player_character::ActiveModel {
                    player_id: ActiveValue::Set(player_id),
                    character_id: ActiveValue::Set(req.item_id),
                    level: ActiveValue::Set(1),
                    experience: ActiveValue::Set(0),
                    play_count: ActiveValue::Set(0),
                }).exec(&db).await?;
            }
            5 // ItCharacter
        }
        ShopItemType::Song => {
            let exists = PlayerBeatmap::find()
                .filter(player_beatmap::Column::PlayerId.eq(player_id))
                .filter(player_beatmap::Column::BeatmapId.eq(req.item_id))
                .one(&db).await?.is_some();
            if !exists {
                PlayerBeatmap::insert(player_beatmap::ActiveModel {
                    player_id: ActiveValue::Set(player_id),
                    beatmap_id: ActiveValue::Set(req.item_id),
                }).exec(&db).await?;
            }
            6 // ItSong
        }
        ShopItemType::Theme => {
            let exists = PlayerTheme::find()
                .filter(player_theme::Column::PlayerId.eq(player_id))
                .filter(player_theme::Column::ThemeId.eq(req.item_id))
                .one(&db).await?.is_some();
            if !exists {
                PlayerTheme::insert(player_theme::ActiveModel {
                    player_id: ActiveValue::Set(player_id),
                    theme_id: ActiveValue::Set(req.item_id),
                }).exec(&db).await?;
            }
            4 // ItTheme
        }
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseShopBuy),
        body: RetShopBuy {
            settle_data: SettleData {
                change_list: vec![SettleItemData { r#type: cost_type, count: price, id: 0 }],
                update_list: vec![SettleItemData { r#type: item_settle_type, count: 1, id: req.item_id }],
                exp_data: None,
            },
        }.encode_to_vec(),
    }])
}
