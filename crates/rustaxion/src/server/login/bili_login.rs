use std::env;
use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{entity::*, query::*};
use tokio::sync::Mutex;

use crate::database::entities::{account, prelude::*};
use crate::types::{response::Response, session::SessionData};

use proto::comet_login::{GatewayServerData, ReqBiliLogin, RetBiliLogin};
use proto::enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqBiliLogin::decode(buffer.as_slice()).context("Failed to decode ReqBiliLogin.")?;

    // NOTE(arjix): We skip the real Bilibili OAuth verification and treat the uid as an identifier.
    // A production implementation would verify the access_key with Bilibili's API first.
    let bili_id = format!("bili:{}", req.uid);
    let token = format!("{:x}", md5::compute(&bili_id));

    let user = Account::find()
        .filter(account::Column::Token.eq(&token))
        .one(&db)
        .await?;

    let acc_id = if let Some(user) = user {
        user.id
    } else {
        let insert = Account::insert(account::ActiveModel {
            steam_id: ActiveValue::Set(bili_id),
            token: ActiveValue::Set(token.clone()),
            is_guest: ActiveValue::Set(false),
            ..Default::default()
        })
        .exec(&db)
        .await?;
        insert.last_insert_id
    };

    let mut session = session.lock().await;
    session.account_id = Some(acc_id);

    let gate_ip = env::var("APP_HOST").unwrap();
    let gate_port: u32 = env::var("APP_PORT").unwrap().parse()?;

    let ret = RetBiliLogin {
        data: GatewayServerData {
            gate_ip,
            gate_port,
            acc_id,
            token,
        },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnBiliLogin),
        body: ret.encode_to_vec(),
    }])
}
