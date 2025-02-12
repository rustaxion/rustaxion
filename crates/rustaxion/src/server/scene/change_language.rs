use crate::types::{response::Response, session::SessionData};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

pub async fn handle(
    _session: &mut SessionData,
    _db: sea_orm::DatabaseConnection,
    body: Vec<u8>,
) -> anyhow::Result<Vec<Response>> {
    // NOTE: The game client sends this request before the player is created.
    //       Meaning, we can't update the player's language here.

    // TODO: Save the language in the session and use it when the player is created.

    // let req = ReqChangeLanguage::decode(body.as_slice())
    //     .context("Failed to decode ReqChangeLanguage.")?;

    // let language = proto::comet_login::LanguageType::from_i32(req.language)
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
