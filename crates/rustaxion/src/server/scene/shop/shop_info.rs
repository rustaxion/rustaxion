use prost::Message;
use sea_orm::EntityTrait;

use crate::{
    database::entities::{prelude::*, sea_orm_active_enums::ShopItemType},
    types::{response::Response, session::SessionData},
};

use proto::comet_scene::{RetShopInfo, ShopRecommend};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(_session: &mut SessionData, db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let shop_items = ShopItem::find().all(&db).await?;

    let character_list = shop_items.iter().filter(|x| x.item_type == ShopItemType::Character).map(|x| x.into_proto()).collect::<Vec<_>>();
    let song_list = shop_items.iter().filter(|x| x.item_type == ShopItemType::Song).map(|x| x.into_proto()).collect::<Vec<_>>();
    let theme_list = shop_items.iter().filter(|x| x.item_type == ShopItemType::Theme).map(|x| x.into_proto()).collect::<Vec<_>>();

    // TODO(arjix): Once a web-ui is made, make this more customizable.
    let ret = RetShopInfo {
        character_list,
        song_list,
        theme_list,
        pay_list: vec![],
        piece_list: vec![],
        member_list: vec![],
        shop_recommend: ShopRecommend { hot_sell_list: vec![] },
        summon_shop_list: vec![],
        vip_reward_list: vec![],
    };
    
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseShopInfo),
        body: ret.encode_to_vec()
    }])
}
