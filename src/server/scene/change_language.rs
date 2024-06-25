use anyhow::Context;
use prost::Message;

use crate::{
    enums::comet::{ comet_scene::CometScene, MainCmd, ParaCmd },
    proto::comet_scene::ReqChangeLanguage,
    types::{ response::Response, session::SessionData },
};

#[rustfmt::skip]
pub async fn handle(session: &mut SessionData, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // let req = ReqChangeLanguage::decode(body.as_slice()).context("Failed to decode ReqChangeLanguage.")?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseChangeLanguage),
        body
    }])
}
