use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::EntityTrait;
use tokio::sync::Mutex;

use crate::{
    database::entities::prelude::*,
    types::{response::Response, session::SessionData},
};

use proto::comet_scene::{PlayerProfileData, ReqSocialPlayerProfile, RetSocialPlayerProfile};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(_session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqSocialPlayerProfile::decode(body.as_slice()).context("Failed to decode ReqSocialPlayerProfile.")?;

    let player = Player::find_by_id(req.char_id as i32).one(&db).await?;
    anyhow::ensure!(player.is_some(), "Player not found");
    let player = player.unwrap();

    let ret = RetSocialPlayerProfile {
        data: PlayerProfileData {
            char_id: player.id as u64,
            name: player.name,
            level: player.level,
            country: player.country.into_proto() as i32,
            is_online: 0,
            list: vec![], // TODO: load dynamics
        },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSocialPlayerProfile),
        body: ret.encode_to_vec(),
    }])
}
