use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use tokio::sync::Mutex;

use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{
    RetBuyProduct, RetIosAppReceipt, RetMissingOrder, RetSendOrder,
    RetTestVerify, RetVerifyGooglePay, RetVerifyIosReceipt, ReqTestVerify, PlayerVipInfo, SettleData,
};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

// Payment handlers are stubs — real implementations would integrate with payment processor APIs.

#[rustfmt::skip]
pub async fn handle_buy_product(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseBuyProduct),
        body: RetBuyProduct {
            order_id: 0,
            sign: None,
            notify_url: None,
        }
        .encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_verify_ios(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseVerifyIOSReceipt),
        body: RetVerifyIosReceipt { status: 0 }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_missing_order(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseMissingOrder),
        body: RetMissingOrder { order_list: vec![] }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_send_order(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSendOrder),
        body: RetSendOrder {
            vip_info: PlayerVipInfo { level: 0, exp: 0, level_up_exp: 0, in_subscription: 0 },
            settle_data: Some(SettleData { change_list: vec![], update_list: vec![], exp_data: None }),
        }
        .encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_verify_google(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseVerifyGooglePay),
        body: RetVerifyGooglePay { status: 0 }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_ios_app_receipt(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseIOSAppReceipt),
        body: RetIosAppReceipt { in_subscription: 0 }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_test_verify(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqTestVerify::decode(body.as_slice()).context("Failed to decode ReqTestVerify.")?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseTestVerify),
        body: RetTestVerify { order_id: req.order_id }.encode_to_vec(),
    }])
}
