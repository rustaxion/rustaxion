# Rustaxion

## About

🚧🚧🚧🚧 <br />
A WIP reverse engineered implementation of [音灵 INVAXION](https://store.steampowered.com/app/921630/_INVAXION/)'s servers, based on the client code.

If you are looking for a working solution, try out my [server emulator](https://github.com/Invaxion-Server-Emulator/invaxion-server-emulator) implemented as a client-side mod for the game.

<!-- progress-start -->
## Progress

<table>
    <thead>
        <th>Comet</th>
        <th>Completion</th>
        <th></th>
    </thead>
    <tbody>
<tr>
    <td>Login</td>
    <td>20%</td>
    <td><table>
<thead>
    <th>Method</th>
    <th>Implemented</th>
</thead>
<tbody>
<tr><td>CometLogin::RequestRegAccount</td><td><input type="checkbox"></td></tr><tr><td>CometLogin::RequestLoginAccount</td><td><input type="checkbox"></td></tr><tr><td>CometLogin::RequestFindPassword</td><td><input type="checkbox"></td></tr><tr><td>CometLogin::RequestQuickToken</td><td><input type="checkbox"></td></tr><tr><td>CometLogin::RequestQuickLogin</td><td><input type="checkbox"></td></tr><tr><td>CometLogin::RequestThirdLogin</td><td><input type="checkbox" checked></td></tr><tr><td>CometLogin::RequestBindAccount</td><td><input type="checkbox"></td></tr><tr><td>CometLogin::RequestAnnouncement</td><td><input type="checkbox"></td></tr><tr><td>CometLogin::RequestGameVersion</td><td><input type="checkbox" checked></td></tr><tr><td>CometLogin::RequestBiliLogin</td><td><input type="checkbox"></td></tr></tbody></table></td>
</tr>
<tr>
    <td>Gate</td>
    <td>50%</td>
    <td><table>
<thead>
    <th>Method</th>
    <th>Implemented</th>
</thead>
<tbody>
<tr><td>CometGate::NotifyGameTime</td><td><input type="checkbox"></td></tr><tr><td>CometGate::RequestUserGameTime</td><td><input type="checkbox"></td></tr><tr><td>CometGate::LoginGateVerify</td><td><input type="checkbox" checked></td></tr><tr><td>CometGate::SelectUserInfoList</td><td><input type="checkbox"></td></tr><tr><td>CometGate::CreateCharacter</td><td><input type="checkbox" checked></td></tr><tr><td>CometGate::EnterGame</td><td><input type="checkbox" checked></td></tr></tbody></table></td>
</tr>
<tr>
    <td>Scene</td>
    <td>3%</td>
    <td><table>
<thead>
    <th>Method</th>
    <th>Implemented</th>
</thead>
<tbody>
<tr><td>CometScene::NotifyCharacterFullData</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestBeginSong</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestFinishSong</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSingleSongRank</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestRankInfo</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSetFavorite</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestBackstageGame</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyUpdateInfo</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestActivityInfo</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestActivityBegin</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestActivityFinish</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyActivityChange</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestMailList</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestGetMailReward</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestDeleteMail</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyDeleteInfo</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestGuide</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestGuideClear</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestChangeHeadIcon</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestChangeCharacter</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestChangeTheme</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestShopInfo</td><td><input type="checkbox" checked></td></tr><tr><td>CometScene::RequestShopBuy</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPieceExchange</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestBattleFieldInfo</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestBattleFieldRankInfo</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestBattleFieldBegin</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestBattleFieldFinish</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSummonInfo</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSummon</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSummonWeekReward</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSummonShopBuy</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestChangeLanguage</td><td><input type="checkbox" checked></td></tr><tr><td>CometScene::RequestSocialSearchPlayer</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSocialPlayerProfile</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSocialSendAddFriendRequest</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifySocialAddFriendRequest</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSocialDeleteFriend</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifySocialDeleteFriend</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSocialDisposeFriendRequest</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifySocialDisposeFriendRequest</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSocialPublishDynamics</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSocialDeleteDynamics</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSocialFriendDynamics</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifySocialFriendPublicDynamic</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifySocialFriendStatus</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestStoryInfo</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestStoryFinish</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestUseItem</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestArcadeInfo</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestArcadeFinish</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestChangeTitle</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestEventInfo</td><td><input type="checkbox" checked></td></tr><tr><td>CometScene::RequestEventLevelGift</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestEventStamina</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestEventNewPlayer</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestEventWeekCheckin</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestEventRecharge</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyRechargeUpdate</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestEventLogin</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestEventNewCharLogin</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestEventNewThemeLogin</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestEventNewCharRelease</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestEventNewThemeRelease</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyEventNewReleaseUpdate</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestEventFriend</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestEventBili</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamCreate</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamSearch</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamList</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamApply</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamDeclaration</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamInfo</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamPosition</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamApplyList</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamDealApply</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamKick</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamExit</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamLogs</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyTeamChange</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyTeamInfoChange</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyTeamApplyChange</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamUploadSong</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamConfirmUploadSong</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTeamBuyItem</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyTeamBuffList</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPreRankInfo</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPreRankBegin</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPreRankEnd</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPreRankRankList</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPVPBeginMatching</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPVPEndMatching</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyPVPMatchSuccess</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPVPMatchConfirm</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyPVPMatchConfirm</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyPVPStartLoading</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPVPFinishLoading</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyPVPFinishLoading</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyPVPStartGame</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPVPSyncScore</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyPVPSyncScore</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPVPUseSkill</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyPVPUseSkill</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPVPFinishGame</td><td><input type="checkbox"></td></tr><tr><td>CometScene::NotifyPVPFinishGame</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestPVPCurrentState</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestBuyProduct</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestVerifyIOSReceipt</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestMissingOrder</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestSendOrder</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestVerifyGooglePay</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestIOSAppReceipt</td><td><input type="checkbox"></td></tr><tr><td>CometScene::RequestTestVerify</td><td><input type="checkbox"></td></tr></tbody></table></td>
</tr>
</tbody></table>
<!-- progress-end -->

## Technologies

Here I list some of the crates I found useful

-   tokio's [runtime](https://github.com/tokio-rs/tokio) as well as its helpers to run a nice TCP server.
-   tokio's [prost](https://github.com/tokio-rs/prost) is used to encode/decode protobuf messages.
-   [enum-repr-derive](https://github.com/ssalonen/enum-repr-derive) to encode/decode enums outside of protobuf.
-   [anyhow](https://github.com/dtolnay/anyhow) to make error handling more convenient

## Contributing

Feel free to contribute in any way you can! <br />
I even accept [donations](https://github.com/sponsors/ArjixWasTaken) if you want to contribute but cannot code :^)

## Attributions

1. [invaxion-server](https://github.com/603185423/invaxion-server) by @MoeGrid and @603185423 for the server logic
2. [Invaxion-Server-Emulator](https://github.com/Invaxion-Server-Emulator/invaxion-server-emulator) by @ArjixWasTaken (me!) for discovering the missing parts (CosmicTour and some other stuff)
   her stuff)
her stuff)
CosmicTour and some other stuff)
   her stuff)
her stuff)
 @603185423 for the server logic
2. [Invaxion-Server-Emulator](https://github.com/Invaxion-Server-Emulator/invaxion-server-emulator) by @ArjixWasTaken (me!) for discovering the missing parts (CosmicTour and some other stuff)
   her stuff)
her stuff)
CosmicTour and some other stuff)
   her stuff)
her stuff)
er stuff)
her stuff)
CosmicTour and some other stuff)
   her stuff)
her stuff)
 stuff)
her stuff)
her stuff)
 stuff)
ff)
her stuff)
CosmicTour and some other stuff)
   her stuff)
her stuff)
er stuff)
her stuff)
CosmicTour and some other stuff)
   her stuff)
her stuff)
 stuff)
her stuff)
her stuff)
 stuff)
Grid and @603185423 for the server logic
2. [Invaxion-Server-Emulator](https://github.com/Invaxion-Server-Emulator/invaxion-server-emulator) by @ArjixWasTaken (me!) for discovering the missing parts (CosmicTour and some other stuff)
   her stuff)
her stuff)
CosmicTour and some other stuff)
   her stuff)
her stuff)
 @603185423 for the server logic
2. [Invaxion-Server-Emulator](https://github.com/Invaxion-Server-Emulator/invaxion-server-emulator) by @ArjixWasTaken (me!) for discovering the missing parts (CosmicTour and some other stuff)
   her stuff)
her stuff)
CosmicTour and some other stuff)
   her stuff)
her stuff)
er stuff)
her stuff)
CosmicTour and some other stuff)
   her stuff)
her stuff)
 stuff)
her stuff)
her stuff)
 stuff)
ff)
her stuff)
CosmicTour and some other stuff)
   her stuff)
her stuff)
er stuff)
her stuff)
CosmicTour and some other stuff)
   her stuff)
her stuff)
 stuff)
her stuff)
her stuff)
 stuff)
