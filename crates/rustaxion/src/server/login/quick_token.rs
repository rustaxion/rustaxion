use std::sync::Arc;

use prost::Message;
use sea_orm::entity::*;
use tokio::sync::Mutex;

use crate::database::entities::{account, prelude::*};
use crate::types::{response::Response, session::SessionData};

use proto::comet_login::RetQuickToken;
use proto::enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(_session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, _buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    // Generate a unique guest token and create a guest account to hold it.
    let token = format!("{:x}", md5::compute(format!("guest:{}", uuid::Uuid::new_v4())));

    Account::insert(account::ActiveModel {
        steam_id: ActiveValue::Set(String::new()),
        token: ActiveValue::Set(token.clone()),
        is_guest: ActiveValue::Set(true),
        ..Default::default()
    })
    .exec(&db)
    .await?;

    let ret = RetQuickToken { token };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnQuickToken),
        body: ret.encode_to_vec(),
    }])
}
