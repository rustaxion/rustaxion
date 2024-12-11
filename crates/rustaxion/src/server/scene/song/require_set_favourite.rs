use anyhow::Context;
use prost::Message;
use sea_orm::{EntityTrait, Set};

use crate::{
    database::entities::{player_favourite_beatmap, prelude::PlayerFavouriteBeatmap},
    enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd},
    proto::comet_scene::ReqSetFavorite,
    types::{response::Response, session::SessionData},
};

#[rustfmt::skip]
pub async fn handle(session: &mut SessionData, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqSetFavorite::decode(body.as_slice()).context("Failed to decode ReqSetFavorite.")?;
    anyhow::ensure!(session.player_id.is_some());

    match req.is_favorite {
        0 => {
            // Remove from favorite
            PlayerFavouriteBeatmap::delete_by_id((session.player_id.unwrap(), req.song_id)).exec(&db).await?;
        },

        1 => {
            // Add to favorite
            PlayerFavouriteBeatmap::insert(player_favourite_beatmap::ActiveModel {
                player_id: Set(session.player_id.unwrap()),
                beatmap_id: Set(req.song_id),
            }).exec(&db).await?;
        },

        _ => anyhow::bail!("Invalid is_favorite value: {}", req.is_favorite),
    }

    Ok(vec![
        Response {
            main_cmd: MainCmd::Game,
            para_cmd: ParaCmd::CometScene(CometScene::ResponseSetFavorite),
            body: body
        }
    ])
}
