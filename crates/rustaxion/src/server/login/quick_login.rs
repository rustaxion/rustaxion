use std::env;
use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{entity::*, query::*};
use tokio::sync::Mutex;

use crate::database::entities::{account, prelude::*};
use crate::types::{response::Response, session::SessionData};

use proto::comet_login::{GatewayServerData, LoginError, ReqQuickLogin, RetQuickLogin};
use proto::enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqQuickLogin::decode(buffer.as_slice()).context("Failed to decode ReqQuickLogin.")?;

    let user = Account::find()
        .filter(account::Column::Token.eq(&req.token))
        .filter(account::Column::IsGuest.eq(true))
        .one(&db)
        .await?;

    let Some(user) = user else {
        let err = LoginError { error_id: 7 }; // TokenError
        return Ok(vec![Response {
            main_cmd: MainCmd::Login,
            para_cmd: ParaCmd::CometLogin(CometLogin::LoginError),
            body: err.encode_to_vec(),
        }]);
    };

    let mut session = session.lock().await;
    session.account_id = Some(user.id);

    let gate_ip = env::var("APP_HOST").unwrap();
    let gate_port: u32 = env::var("APP_PORT").unwrap().parse()?;

    let ret = RetQuickLogin {
        data: GatewayServerData {
            gate_ip,
            gate_port,
            acc_id: user.id,
            token: user.token,
        },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnQuickLogin),
        body: ret.encode_to_vec(),
    }])
}
