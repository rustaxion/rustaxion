use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{entity::*, query::*};
use tokio::sync::Mutex;

use crate::database::entities::{friend, prelude::*, sea_orm_active_enums::FriendStatus};
use crate::types::{response::Response, session::SessionData};

use proto::comet_scene::{
    ReqSocialDelDynamics, ReqSocialDelFriend, ReqSocialDisposeFriendRequest,
    ReqSocialFriendDynamics, ReqSocialSendAddFriendRequest, RetSocialDelDynamics,
    RetSocialDelFriend, RetSocialDisposeFriendRequest, RetSocialFriendDynamics,
    RetSocialSendAddFriendRequest,
};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle_send_friend_request(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqSocialSendAddFriendRequest::decode(body.as_slice()).context("Failed to decode ReqSocialSendAddFriendRequest.")?;
    let player_id = session.lock().await.player_id.context("Not logged in")?;
    let target_id = req.char_id as i32;

    let already_exists = Friend::find()
        .filter(friend::Column::PlayerId.eq(player_id))
        .filter(friend::Column::FriendId.eq(target_id))
        .one(&db).await?;

    if already_exists.is_none() {
        Friend::insert(friend::ActiveModel {
            player_id: ActiveValue::Set(player_id),
            friend_id: ActiveValue::Set(target_id),
            status: ActiveValue::Set(FriendStatus::Pending),
            created_at: ActiveValue::Set(chrono::Utc::now().into()),
            ..Default::default()
        }).exec(&db).await?;
    }

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSocialSendAddFriendRequest),
        body: RetSocialSendAddFriendRequest { char_id: req.char_id }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_delete_friend(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqSocialDelFriend::decode(body.as_slice()).context("Failed to decode ReqSocialDelFriend.")?;
    let player_id = session.lock().await.player_id.context("Not logged in")?;
    let target_id = req.char_id as i32;

    // Delete both directions of the friendship
    Friend::delete_many()
        .filter(
            Condition::any()
                .add(Condition::all()
                    .add(friend::Column::PlayerId.eq(player_id))
                    .add(friend::Column::FriendId.eq(target_id)))
                .add(Condition::all()
                    .add(friend::Column::PlayerId.eq(target_id))
                    .add(friend::Column::FriendId.eq(player_id)))
        )
        .exec(&db).await?;

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSocialDeleteFriend),
        body: RetSocialDelFriend { char_id: req.char_id }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_dispose_friend_request(session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqSocialDisposeFriendRequest::decode(body.as_slice()).context("Failed to decode ReqSocialDisposeFriendRequest.")?;
    let player_id = session.lock().await.player_id.context("Not logged in")?;
    let requester_id = req.char_id as i32;

    // Find the pending request from requester to us
    let pending = Friend::find()
        .filter(friend::Column::PlayerId.eq(requester_id))
        .filter(friend::Column::FriendId.eq(player_id))
        .filter(friend::Column::Status.eq(FriendStatus::Pending))
        .one(&db).await?;

    if let Some(row) = pending {
        if req.is_accept == 1 {
            // Accept: mark existing row as accepted, insert reverse
            let mut active: friend::ActiveModel = row.into();
            active.status = ActiveValue::Set(FriendStatus::Accepted);
            active.update(&db).await?;

            Friend::insert(friend::ActiveModel {
                player_id: ActiveValue::Set(player_id),
                friend_id: ActiveValue::Set(requester_id),
                status: ActiveValue::Set(FriendStatus::Accepted),
                created_at: ActiveValue::Set(chrono::Utc::now().into()),
                ..Default::default()
            }).exec(&db).await?;
        } else {
            // Reject: delete the pending row
            Friend::delete_by_id(row.id).exec(&db).await?;
        }
    }

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSocialDisposeFriendRequest),
        body: RetSocialDisposeFriendRequest { char_id: req.char_id }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_delete_dynamics(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqSocialDelDynamics::decode(body.as_slice()).context("Failed to decode ReqSocialDelDynamics.")?;
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSocialDeleteDynamics),
        body: RetSocialDelDynamics { index: req.index }.encode_to_vec(),
    }])
}

#[rustfmt::skip]
pub async fn handle_friend_dynamics(_session: Arc<Mutex<SessionData>>, _db: sea_orm::DatabaseConnection, _body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let _ = ReqSocialFriendDynamics {};
    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSocialFriendDynamics),
        body: RetSocialFriendDynamics { list: vec![] }.encode_to_vec(),
    }])
}
