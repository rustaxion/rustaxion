use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::sync::Mutex;

use crate::{
    database::entities::{player, prelude::*},
    types::{response::Response, session::SessionData},
};

use proto::comet_scene::{ReqSocialSearchPlayer, RetSocialSearchPlayer};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(_session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqSocialSearchPlayer::decode(body.as_slice()).context("Failed to decode ReqSocialSearchPlayer.")?;

    let found = Player::find()
        .filter(player::Column::Name.eq(&req.name))
        .one(&db)
        .await?;

    let ret = match found {
        Some(p) => RetSocialSearchPlayer {
            result: 1,
            char_id: Some(p.id as u64),
            name: Some(p.name),
            head_id: Some(p.head_id),
            country: Some(p.country.into_proto() as i32),
            is_online: Some(0),
        },
        None => RetSocialSearchPlayer {
            result: 0,
            char_id: None,
            name: None,
            head_id: None,
            country: None,
            is_online: None,
        },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSocialSearchPlayer),
        body: ret.encode_to_vec(),
    }])
}
