use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use tokio::sync::Mutex;

use crate::{
    database::entities::{player, player_character, player_theme, prelude::*},
    types::{response::Response, session::SessionData},
};

use proto::comet_scene::{
    ReqChangeCharacter, ReqChangeHeadIcon, ReqChangeTheme, ReqChangeTitle, RetChangeCharacter,
    RetChangeHeadIcon, RetChangeTheme, RetChangeTitle,
};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle_head_icon(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqChangeHeadIcon::decode(body.as_slice()).context("Failed to decode ReqChangeHeadIcon.")?;

    let session = session.lock().await;
    anyhow::ensure!(session.player_id.is_some());

    player::ActiveModel {
        id: ActiveValue::Unchanged(session.player_id.unwrap()),
        head_id: ActiveValue::Set(req.id),
        ..Default::default()
    }
    .update(&db)
    .await?;

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseChangeHeadIcon),
        body: RetChangeHeadIcon { id: req.id }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_character(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqChangeCharacter::decode(body.as_slice()).context("Failed to decode ReqChangeCharacter.")?;

    let session = session.lock().await;
    anyhow::ensure!(session.player_id.is_some());
    let player_id = session.player_id.unwrap();

    // Verify ownership
    let owned = PlayerCharacter::find()
        .filter(player_character::Column::PlayerId.eq(player_id))
        .filter(player_character::Column::CharacterId.eq(req.id))
        .one(&db)
        .await?;
    anyhow::ensure!(owned.is_some(), "Player does not own character {}", req.id);

    player::ActiveModel {
        id: ActiveValue::Unchanged(player_id),
        selected_character_id: ActiveValue::Set(req.id),
        ..Default::default()
    }
    .update(&db)
    .await?;

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseChangeCharacter),
        body: RetChangeCharacter { id: req.id }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_theme(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqChangeTheme::decode(body.as_slice()).context("Failed to decode ReqChangeTheme.")?;

    let session = session.lock().await;
    anyhow::ensure!(session.player_id.is_some());
    let player_id = session.player_id.unwrap();

    // Verify ownership
    let owned = PlayerTheme::find()
        .filter(player_theme::Column::PlayerId.eq(player_id))
        .filter(player_theme::Column::ThemeId.eq(req.id))
        .one(&db)
        .await?;
    anyhow::ensure!(owned.is_some(), "Player does not own theme {}", req.id);

    player::ActiveModel {
        id: ActiveValue::Unchanged(player_id),
        selected_theme_id: ActiveValue::Set(req.id),
        ..Default::default()
    }
    .update(&db)
    .await?;

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseChangeTheme),
        body: RetChangeTheme { id: req.id }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_title(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqChangeTitle::decode(body.as_slice()).context("Failed to decode ReqChangeTitle.")?;

    let session = session.lock().await;
    anyhow::ensure!(session.player_id.is_some());

    player::ActiveModel {
        id: ActiveValue::Unchanged(session.player_id.unwrap()),
        title_id: ActiveValue::Set(req.title_id),
        ..Default::default()
    }
    .update(&db)
    .await?;

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseChangeTitle),
        body: RetChangeTitle { title_id: req.title_id }.encode_to_vec(),
    }])
}
