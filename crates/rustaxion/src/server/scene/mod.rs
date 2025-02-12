use crate::types::{response::Response, session::SessionData};
use proto::{
    enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd},
    packet::Packet,
};

mod change_language;
mod event;
mod rank_info;
mod shop;
mod social;
mod song;

#[rustfmt::skip]
pub async fn handle(
    session: &mut SessionData,
    db: sea_orm::DatabaseConnection,
    Packet {
        main_cmd,
        para_cmd,
        data,
        ..
    }: Packet,
) -> anyhow::Result<Vec<Response>> {
    assert_eq!(main_cmd, MainCmd::Game);
    let ParaCmd::CometScene(para_cmd) = para_cmd else {
        anyhow::bail!("How did we get here?")
    };

    match para_cmd {
        CometScene::RequestBeginSong => song::begin_song::handle(session, db, data).await,
        CometScene::RequestFinishSong => song::finish_song::handle(session, db, data).await,
        CometScene::RequestSingleSongRank => todo!(),
        CometScene::RequestRankInfo => rank_info::handle(session, db, data).await,
        CometScene::RequestSetFavorite => song::require_set_favourite::handle(session, db, data).await,
        CometScene::RequestBackstageGame => todo!(),
        CometScene::RequestActivityInfo => todo!(),
        CometScene::RequestActivityBegin => todo!(),
        CometScene::RequestActivityFinish => todo!(),
        CometScene::RequestMailList => todo!(),
        CometScene::RequestGetMailReward => todo!(),
        CometScene::RequestDeleteMail => todo!(),
        CometScene::RequestGuide => todo!(),
        CometScene::RequestGuideClear => todo!(),
        CometScene::RequestChangeHeadIcon => todo!(),
        CometScene::RequestChangeCharacter => todo!(),
        CometScene::RequestChangeTheme => todo!(),
        CometScene::RequestShopInfo => shop::shop_info::handle(session, db, data).await,
        CometScene::RequestShopBuy => todo!(),
        CometScene::RequestPieceExchange => todo!(),
        CometScene::RequestBattleFieldInfo => todo!(),
        CometScene::RequestBattleFieldRankInfo => todo!(),
        CometScene::RequestBattleFieldBegin => todo!(),
        CometScene::RequestBattleFieldFinish => todo!(),
        CometScene::RequestSummonInfo => todo!(),
        CometScene::RequestSummon => todo!(),
        CometScene::RequestSummonWeekReward => todo!(),
        CometScene::RequestSummonShopBuy => todo!(),
        CometScene::RequestChangeLanguage => change_language::handle(session, db, data).await,
        CometScene::RequestSocialSearchPlayer => todo!(),
        CometScene::RequestSocialPlayerProfile => todo!(),
        CometScene::RequestSocialSendAddFriendRequest => todo!(),
        CometScene::RequestSocialDeleteFriend => todo!(),
        CometScene::RequestSocialDisposeFriendRequest => todo!(),
        CometScene::RequestSocialPublishDynamics => social::publish_dynamics::handle(session, db, data).await,
        CometScene::RequestSocialDeleteDynamics => todo!(),
        CometScene::RequestSocialFriendDynamics => todo!(),
        CometScene::RequestStoryInfo => todo!(),
        CometScene::RequestStoryFinish => todo!(),
        CometScene::RequestUseItem => todo!(),
        CometScene::RequestArcadeInfo => todo!(),
        CometScene::RequestArcadeFinish => todo!(),
        CometScene::RequestChangeTitle => todo!(),
        CometScene::RequestEventInfo => event::event_info::handle(session, db, data).await,
        CometScene::RequestEventLevelGift => todo!(),
        CometScene::RequestEventStamina => todo!(),
        CometScene::RequestEventNewPlayer => todo!(),
        CometScene::RequestEventWeekCheckin => todo!(),
        CometScene::RequestEventRecharge => todo!(),
        CometScene::RequestEventLogin => todo!(),
        CometScene::RequestEventNewCharLogin => todo!(),
        CometScene::RequestEventNewThemeLogin => todo!(),
        CometScene::RequestEventNewCharRelease => todo!(),
        CometScene::RequestEventNewThemeRelease => todo!(),
        CometScene::RequestEventFriend => todo!(),
        CometScene::RequestEventBili => todo!(),
        CometScene::RequestTeamCreate => todo!(),
        CometScene::RequestTeamSearch => todo!(),
        CometScene::RequestTeamList => todo!(),
        CometScene::RequestTeamApply => todo!(),
        CometScene::RequestTeamDeclaration => todo!(),
        CometScene::RequestTeamInfo => todo!(),
        CometScene::RequestTeamPosition => todo!(),
        CometScene::RequestTeamApplyList => todo!(),
        CometScene::RequestTeamDealApply => todo!(),
        CometScene::RequestTeamKick => todo!(),
        CometScene::RequestTeamExit => todo!(),
        CometScene::RequestTeamLogs => todo!(),
        CometScene::RequestTeamUploadSong => todo!(),
        CometScene::RequestTeamConfirmUploadSong => todo!(),
        CometScene::RequestTeamBuyItem => todo!(),
        CometScene::RequestPreRankInfo => todo!(),
        CometScene::RequestPreRankBegin => todo!(),
        CometScene::RequestPreRankEnd => todo!(),
        CometScene::RequestPreRankRankList => todo!(),
        CometScene::RequestPVPBeginMatching => todo!(),
        CometScene::RequestPVPEndMatching => todo!(),
        CometScene::RequestPVPMatchConfirm => todo!(),
        CometScene::RequestPVPFinishLoading => todo!(),
        CometScene::RequestPVPSyncScore => todo!(),
        CometScene::RequestPVPUseSkill => todo!(),
        CometScene::RequestPVPFinishGame => todo!(),
        CometScene::RequestPVPCurrentState => todo!(),
        CometScene::RequestBuyProduct => todo!(),
        CometScene::RequestVerifyIOSReceipt => todo!(),
        CometScene::RequestMissingOrder => todo!(),
        CometScene::RequestSendOrder => todo!(),
        CometScene::RequestVerifyGooglePay => todo!(),
        CometScene::RequestIOSAppReceipt => todo!(),
        CometScene::RequestTestVerify => todo!(),

        // NOTE(arjix): When given a client-side param, what should we do?
        _ => unreachable!()
    }
}
