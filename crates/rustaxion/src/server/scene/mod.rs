use std::sync::Arc;

use crate::types::{response::Response, session::SessionData};
use proto::{
    enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd},
    packet::Packet,
};
use tokio::sync::Mutex;

mod activity;
mod arcade;
mod battlefield;
mod change_language;
mod change_player_info;
mod event;
mod guide;
mod mail;
mod payment;
mod piece_exchange;
mod prerank;
mod pvp;
mod rank_info;
mod shop;
mod single_song_rank;
mod social;
mod song;
mod story;
mod summon;
mod team;
mod use_item;

#[rustfmt::skip]
pub async fn handle(
    session: Arc<Mutex<SessionData>>,
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
        CometScene::RequestSingleSongRank => single_song_rank::handle(session, db, data).await,
        CometScene::RequestRankInfo => rank_info::handle(session, db, data).await,
        CometScene::RequestSetFavorite => song::require_set_favourite::handle(session, db, data).await,
        CometScene::RequestBackstageGame => Ok(vec![]),
        CometScene::RequestActivityInfo => activity::handle_info(session, db, data).await,
        CometScene::RequestActivityBegin => activity::handle_begin(session, db, data).await,
        CometScene::RequestActivityFinish => activity::handle_finish(session, db, data).await,
        CometScene::RequestMailList => mail::handle_list(session, db, data).await,
        CometScene::RequestGetMailReward => mail::handle_get_reward(session, db, data).await,
        CometScene::RequestDeleteMail => mail::handle_delete(session, db, data).await,
        CometScene::RequestGuide => guide::handle_guide(session, db, data).await,
        CometScene::RequestGuideClear => guide::handle_guide_clear(session, db, data).await,
        CometScene::RequestChangeHeadIcon => change_player_info::handle_head_icon(session, db, data).await,
        CometScene::RequestChangeCharacter => change_player_info::handle_character(session, db, data).await,
        CometScene::RequestChangeTheme => change_player_info::handle_theme(session, db, data).await,
        CometScene::RequestShopInfo => shop::shop_info::handle(session, db, data).await,
        CometScene::RequestShopBuy => shop::shop_buy::handle(session, db, data).await,
        CometScene::RequestPieceExchange => piece_exchange::handle(session, db, data).await,
        CometScene::RequestBattleFieldInfo => battlefield::handle_info(session, db, data).await,
        CometScene::RequestBattleFieldRankInfo => battlefield::handle_rank_info(session, db, data).await,
        CometScene::RequestBattleFieldBegin => battlefield::handle_begin(session, db, data).await,
        CometScene::RequestBattleFieldFinish => battlefield::handle_finish(session, db, data).await,
        CometScene::RequestSummonInfo => summon::handle_info(session, db, data).await,
        CometScene::RequestSummon => summon::handle_summon(session, db, data).await,
        CometScene::RequestSummonWeekReward => summon::handle_week_reward(session, db, data).await,
        CometScene::RequestSummonShopBuy => summon::handle_shop_buy(session, db, data).await,
        CometScene::RequestChangeLanguage => change_language::handle(session, db, data).await,
        CometScene::RequestSocialSearchPlayer => social::search_player::handle(session, db, data).await,
        CometScene::RequestSocialPlayerProfile => social::player_profile::handle(session, db, data).await,
        CometScene::RequestSocialSendAddFriendRequest => social::friends::handle_send_friend_request(session, db, data).await,
        CometScene::RequestSocialDeleteFriend => social::friends::handle_delete_friend(session, db, data).await,
        CometScene::RequestSocialDisposeFriendRequest => social::friends::handle_dispose_friend_request(session, db, data).await,
        CometScene::RequestSocialPublishDynamics => social::publish_dynamics::handle(session, db, data).await,
        CometScene::RequestSocialDeleteDynamics => social::friends::handle_delete_dynamics(session, db, data).await,
        CometScene::RequestSocialFriendDynamics => social::friends::handle_friend_dynamics(session, db, data).await,
        CometScene::RequestStoryInfo => story::handle_info(session, db, data).await,
        CometScene::RequestStoryFinish => story::handle_finish(session, db, data).await,
        CometScene::RequestUseItem => use_item::handle(session, db, data).await,
        CometScene::RequestArcadeInfo => arcade::handle_info(session, db, data).await,
        CometScene::RequestArcadeFinish => arcade::handle_finish(session, db, data).await,
        CometScene::RequestChangeTitle => change_player_info::handle_title(session, db, data).await,
        CometScene::RequestEventInfo => event::event_info::handle(session, db, data).await,
        CometScene::RequestEventLevelGift => event::claim::handle_level_gift(session, db, data).await,
        CometScene::RequestEventStamina => event::claim::handle_stamina(session, db, data).await,
        CometScene::RequestEventNewPlayer => event::claim::handle_new_player(session, db, data).await,
        CometScene::RequestEventWeekCheckin => event::claim::handle_week_checkin(session, db, data).await,
        CometScene::RequestEventRecharge => event::claim::handle_recharge(session, db, data).await,
        CometScene::RequestEventLogin => event::claim::handle_login(session, db, data).await,
        CometScene::RequestEventNewCharLogin => event::claim::handle_new_char_login(session, db, data).await,
        CometScene::RequestEventNewThemeLogin => event::claim::handle_new_theme_login(session, db, data).await,
        CometScene::RequestEventNewCharRelease => event::claim::handle_new_char_release(session, db, data).await,
        CometScene::RequestEventNewThemeRelease => event::claim::handle_new_theme_release(session, db, data).await,
        CometScene::RequestEventFriend => event::claim::handle_friend(session, db, data).await,
        CometScene::RequestEventBili => event::claim::handle_bili(session, db, data).await,
        CometScene::RequestTeamCreate => team::handle_create(session, db, data).await,
        CometScene::RequestTeamSearch => team::handle_search(session, db, data).await,
        CometScene::RequestTeamList => team::handle_list(session, db, data).await,
        CometScene::RequestTeamApply => team::handle_apply(session, db, data).await,
        CometScene::RequestTeamDeclaration => team::handle_declaration(session, db, data).await,
        CometScene::RequestTeamInfo => team::handle_info(session, db, data).await,
        CometScene::RequestTeamPosition => team::handle_position(session, db, data).await,
        CometScene::RequestTeamApplyList => team::handle_apply_list(session, db, data).await,
        CometScene::RequestTeamDealApply => team::handle_deal_apply(session, db, data).await,
        CometScene::RequestTeamKick => team::handle_kick(session, db, data).await,
        CometScene::RequestTeamExit => team::handle_exit(session, db, data).await,
        CometScene::RequestTeamLogs => team::handle_logs(session, db, data).await,
        CometScene::RequestTeamUploadSong => team::handle_upload_song(session, db, data).await,
        CometScene::RequestTeamConfirmUploadSong => team::handle_confirm_upload_song(session, db, data).await,
        CometScene::RequestTeamBuyItem => team::handle_buy_item(session, db, data).await,
        CometScene::RequestPreRankInfo => prerank::handle_info(session, db, data).await,
        CometScene::RequestPreRankBegin => prerank::handle_begin(session, db, data).await,
        CometScene::RequestPreRankEnd => prerank::handle_end(session, db, data).await,
        CometScene::RequestPreRankRankList => prerank::handle_rank_list(session, db, data).await,
        CometScene::RequestPVPBeginMatching => pvp::handle_begin_matching(session, db, data).await,
        CometScene::RequestPVPEndMatching => pvp::handle_end_matching(session, db, data).await,
        CometScene::RequestPVPMatchConfirm => pvp::handle_match_confirm(session, db, data).await,
        CometScene::RequestPVPFinishLoading => pvp::handle_finish_loading(session, db, data).await,
        CometScene::RequestPVPSyncScore => pvp::handle_sync_score(session, db, data).await,
        CometScene::RequestPVPUseSkill => pvp::handle_use_skill(session, db, data).await,
        CometScene::RequestPVPFinishGame => pvp::handle_finish_game(session, db, data).await,
        CometScene::RequestPVPCurrentState => pvp::handle_current_state(session, db, data).await,
        CometScene::RequestBuyProduct => payment::handle_buy_product(session, db, data).await,
        CometScene::RequestVerifyIOSReceipt => payment::handle_verify_ios(session, db, data).await,
        CometScene::RequestMissingOrder => payment::handle_missing_order(session, db, data).await,
        CometScene::RequestSendOrder => payment::handle_send_order(session, db, data).await,
        CometScene::RequestVerifyGooglePay => payment::handle_verify_google(session, db, data).await,
        CometScene::RequestIOSAppReceipt => payment::handle_ios_app_receipt(session, db, data).await,
        CometScene::RequestTestVerify => payment::handle_test_verify(session, db, data).await,

        // NOTE(arjix): When given a client-side param, what should we do?
        _ => unreachable!()
    }
}
