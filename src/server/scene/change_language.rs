use crate::database::entities::{player, prelude::*};
use anyhow::Context;
use prost::Message;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
    database::entities::sea_orm_active_enums,
    enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd},
    proto::comet_scene::ReqChangeLanguage,
    types::{response::Response, session::SessionData},
};

pub async fn handle(
    session: &mut SessionData,
    db: sea_orm::DatabaseConnection,
    body: Vec<u8>,
) -> anyhow::Result<Vec<Response>> {
    // NOTE: The game client sends this request before the player is created.
    //       Meaning, we can't update the player's language here.

    // TODO: Save the language in the session and use it when the player is created.

    // let req = ReqChangeLanguage::decode(body.as_slice())
    //     .context("Failed to decode ReqChangeLanguage.")?;

    // let language = crate::proto::comet_login::LanguageType::from_i32(req.language)
    //     .ok_or(anyhow::anyhow!("Invalid language type."))?;

    // Player::update(player::ActiveModel {
    //     language: Set(sea_orm_active_enums::Language::from_proto(language)),
    //     ..Default::default()
    // })
    // .filter(player::Column::Id.eq(session.player_id.unwrap()))
    // .exec(&db)
    // .await?;

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseChangeLanguage),
        body: body,
    }])
}
