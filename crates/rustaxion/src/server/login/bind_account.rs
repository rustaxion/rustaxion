use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{entity::*, query::*};
use tokio::sync::Mutex;

use crate::database::entities::{account, prelude::*};
use crate::types::{response::Response, session::SessionData};

use proto::comet_login::{LoginError, ReqBindAccount, RetBindAccount};
use proto::enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(_session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqBindAccount::decode(buffer.as_slice()).context("Failed to decode ReqBindAccount.")?;

    // Resolve guest account by token
    let user = Account::find()
        .filter(account::Column::Token.eq(&req.token))
        .filter(account::Column::IsGuest.eq(true))
        .one(&db)
        .await?;

    let Some(user) = user else {
        let err = LoginError { error_id: 7 }; // TokenError / invalid token
        return Ok(vec![Response {
            main_cmd: MainCmd::Login,
            para_cmd: ParaCmd::CometLogin(CometLogin::LoginError),
            body: err.encode_to_vec(),
        }]);
    };

    // Check the desired account name is not already taken
    let name_taken = Account::find()
        .filter(account::Column::AccountName.eq(&req.account_name))
        .one(&db)
        .await?;

    if name_taken.is_some() {
        let err = LoginError { error_id: 2 }; // AccountAlreadyExists
        return Ok(vec![Response {
            main_cmd: MainCmd::Login,
            para_cmd: ParaCmd::CometLogin(CometLogin::LoginError),
            body: err.encode_to_vec(),
        }]);
    }

    let password_hash = format!("{:x}", md5::compute(&req.password));
    account::ActiveModel {
        id: ActiveValue::Unchanged(user.id),
        account_name: ActiveValue::Set(Some(req.account_name)),
        password_hash: ActiveValue::Set(Some(password_hash)),
        is_guest: ActiveValue::Set(false),
        ..Default::default()
    }
    .update(&db)
    .await?;

    let ret = RetBindAccount {};

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnBindAccount),
        body: ret.encode_to_vec(),
    }])
}
