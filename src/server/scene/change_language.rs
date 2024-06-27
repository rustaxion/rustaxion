use crate::{
    enums::comet::{ comet_scene::CometScene, MainCmd, ParaCmd },
    types::{ response::Response, session::SessionData },
};

#[rustfmt::skip]
pub async fn handle(_session: &mut SessionData, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // let req = ReqChangeLanguage::decode(body.as_slice()).context("Failed to decode ReqChangeLanguage.")?;

    // NOTE(arjix): We are ignoring the client's request for a language change, because we don't know if this data is useful or not.

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseChangeLanguage),
        body: vec![]
    }])
}
