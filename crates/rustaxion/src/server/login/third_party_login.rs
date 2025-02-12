use std::env;

use anyhow::Context;
use prost::Message;

use crate::database::entities::{account, prelude::*};
use sea_orm::{entity::*, query::*};

use crate::types::{response::Response, session::SessionData};

use proto::comet_login::{GatewayServerData, ReqThirdLogin, RetThirdLogin};
use proto::enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(session: &mut SessionData, db: sea_orm::DatabaseConnection, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqThirdLogin::decode(buffer.as_slice()).context("Failed to decode ReqThirdLogin.")?;
    let token = format!("{:x}", md5::compute(req.clone().open_id + "6031"));

    let user = Account::find().filter(account::Column::Token.eq(&token)).one(&db).await?;
    let mut acc_id = user.clone().map(|x| x.id);
    if user.is_none() {
        let insert = Account::insert(account::ActiveModel {
            token: ActiveValue::Set(token.clone()),
            steam_id: ActiveValue::Set(req.open_id),
            ..Default::default()
        }).exec(&db).await?;

        acc_id = Some(insert.last_insert_id);
    }

    anyhow::ensure!(acc_id.is_some());
    let acc_id = acc_id.unwrap();
    session.account_id = Some(acc_id);

    let gate_ip = env::var("APP_HOST").unwrap();
    let gate_port: u32 = env::var("APP_PORT").unwrap().parse()?;

    let ret = RetThirdLogin {
        data: GatewayServerData {
            gate_ip,
            gate_port,
            acc_id,
            token,
        }
    };

    dbg!(&ret);

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnThirdLogin),
        body: ret.encode_to_vec()
    }])
}
