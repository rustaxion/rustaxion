use std::env;
use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{entity::*, query::*};
use tokio::sync::Mutex;

use crate::database::entities::{account, prelude::*};
use crate::types::{response::Response, session::SessionData};

use proto::comet_login::{GatewayServerData, LoginError, ReqLoginAccount, RetLoginAccount};
use proto::enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqLoginAccount::decode(buffer.as_slice()).context("Failed to decode ReqLoginAccount.")?;

    let user = Account::find()
        .filter(account::Column::AccountName.eq(&req.account_name))
        .one(&db)
        .await?;

    let Some(user) = user else {
        let err = LoginError { error_id: 4 }; // AccountNotFound
        return Ok(vec![Response {
            main_cmd: MainCmd::Login,
            para_cmd: ParaCmd::CometLogin(CometLogin::LoginError),
            body: err.encode_to_vec(),
        }]);
    };

    let password_hash = format!("{:x}", md5::compute(&req.password));
    if user.password_hash.as_deref() != Some(&password_hash) {
        let err = LoginError { error_id: 5 }; // PasswordError
        return Ok(vec![Response {
            main_cmd: MainCmd::Login,
            para_cmd: ParaCmd::CometLogin(CometLogin::LoginError),
            body: err.encode_to_vec(),
        }]);
    }

    let mut session = session.lock().await;
    session.account_id = Some(user.id);

    let gate_ip = env::var("APP_HOST").unwrap();
    let gate_port: u32 = env::var("APP_PORT").unwrap().parse()?;

    let ret = RetLoginAccount {
        data: GatewayServerData {
            gate_ip,
            gate_port,
            acc_id: user.id,
            token: user.token,
        },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnLoginAccount),
        body: ret.encode_to_vec(),
    }])
}
