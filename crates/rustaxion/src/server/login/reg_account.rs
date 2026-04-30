use std::env;
use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{entity::*, query::*};
use tokio::sync::Mutex;

use crate::database::entities::{account, prelude::*};
use crate::types::{response::Response, session::SessionData};

use proto::comet_login::{GatewayServerData, LoginError, ReqRegAccount, RetRegAccount};
use proto::enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqRegAccount::decode(buffer.as_slice()).context("Failed to decode ReqRegAccount.")?;

    // Check if account_name already taken
    let existing = Account::find()
        .filter(account::Column::AccountName.eq(&req.account_name))
        .one(&db)
        .await?;

    if existing.is_some() {
        let err = LoginError { error_id: 2 }; // AccountAlreadyExists
        return Ok(vec![Response {
            main_cmd: MainCmd::Login,
            para_cmd: ParaCmd::CometLogin(CometLogin::LoginError),
            body: err.encode_to_vec(),
        }]);
    }

    // TODO(arjix): Use a proper password hashing algorithm (bcrypt/argon2).
    let password_hash = format!("{:x}", md5::compute(&req.password));
    let token = format!("{:x}", md5::compute(format!("reg:{}:{}", &req.account_name, &req.password)));

    let insert = Account::insert(account::ActiveModel {
        steam_id: ActiveValue::Set(String::new()),
        token: ActiveValue::Set(token.clone()),
        account_name: ActiveValue::Set(Some(req.account_name)),
        mail: ActiveValue::Set(Some(req.mail)),
        password_hash: ActiveValue::Set(Some(password_hash)),
        is_guest: ActiveValue::Set(false),
        ..Default::default()
    })
    .exec(&db)
    .await?;

    let acc_id = insert.last_insert_id;

    let mut session = session.lock().await;
    session.account_id = Some(acc_id);

    let gate_ip = env::var("APP_HOST").unwrap();
    let gate_port: u32 = env::var("APP_PORT").unwrap().parse()?;

    let ret = RetRegAccount {
        data: GatewayServerData {
            gate_ip,
            gate_port,
            acc_id,
            token,
        },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnRegAccount),
        body: ret.encode_to_vec(),
    }])
}
