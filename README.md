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
<tbody>
<tr><td>CometLogin::RequestRegAccount <ul><li>- [ ]</li></ul></td></tr><tr><td>CometLogin::RequestLoginAccount <ul><li>- [ ]</li></ul></td></tr><tr><td>CometLogin::RequestFindPassword <ul><li>- [ ]</li></ul></td></tr><tr><td>CometLogin::RequestQuickToken <ul><li>- [ ]</li></ul></td></tr><tr><td>CometLogin::RequestQuickLogin <ul><li>- [ ]</li></ul></td></tr><tr><td>CometLogin::RequestThirdLogin <ul><li>- [x]</li></ul></td></tr><tr><td>CometLogin::RequestBindAccount <ul><li>- [ ]</li></ul></td></tr><tr><td>CometLogin::RequestAnnouncement <ul><li>- [ ]</li></ul></td></tr><tr><td>CometLogin::RequestGameVersion <ul><li>- [x]</li></ul></td></tr><tr><td>CometLogin::RequestBiliLogin <ul><li>- [ ]</li></ul></td></tr></tbody></table></td>
</tr>
<tr>
    <td>Gate</td>
    <td>50%</td>
    <td><table>
<tbody>
<tr><td>CometGate::NotifyGameTime <ul><li>- [ ]</li></ul></td></tr><tr><td>CometGate::RequestUserGameTime <ul><li>- [ ]</li></ul></td></tr><tr><td>CometGate::LoginGateVerify <ul><li>- [x]</li></ul></td></tr><tr><td>CometGate::SelectUserInfoList <ul><li>- [ ]</li></ul></td></tr><tr><td>CometGate::CreateCharacter <ul><li>- [x]</li></ul></td></tr><tr><td>CometGate::EnterGame <ul><li>- [x]</li></ul></td></tr></tbody></table></td>
</tr>
<tr>
    <td>Scene</td>
    <td>3%</td>
    <td><table>
<tbody>
<tr><td>CometScene::NotifyCharacterFullData <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestBeginSong <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestFinishSong <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSingleSongRank <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestRankInfo <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSetFavorite <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestBackstageGame <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyUpdateInfo <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestActivityInfo <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestActivityBegin <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestActivityFinish <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyActivityChange <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestMailList <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestGetMailReward <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestDeleteMail <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyDeleteInfo <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestGuide <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestGuideClear <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestChangeHeadIcon <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestChangeCharacter <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestChangeTheme <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestShopInfo <ul><li>- [x]</li></ul></td></tr><tr><td>CometScene::RequestShopBuy <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPieceExchange <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestBattleFieldInfo <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestBattleFieldRankInfo <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestBattleFieldBegin <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestBattleFieldFinish <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSummonInfo <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSummon <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSummonWeekReward <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSummonShopBuy <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestChangeLanguage <ul><li>- [x]</li></ul></td></tr><tr><td>CometScene::RequestSocialSearchPlayer <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSocialPlayerProfile <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSocialSendAddFriendRequest <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifySocialAddFriendRequest <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSocialDeleteFriend <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifySocialDeleteFriend <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSocialDisposeFriendRequest <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifySocialDisposeFriendRequest <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSocialPublishDynamics <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSocialDeleteDynamics <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSocialFriendDynamics <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifySocialFriendPublicDynamic <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifySocialFriendStatus <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestStoryInfo <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestStoryFinish <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestUseItem <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestArcadeInfo <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestArcadeFinish <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestChangeTitle <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestEventInfo <ul><li>- [x]</li></ul></td></tr><tr><td>CometScene::RequestEventLevelGift <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestEventStamina <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestEventNewPlayer <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestEventWeekCheckin <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestEventRecharge <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyRechargeUpdate <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestEventLogin <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestEventNewCharLogin <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestEventNewThemeLogin <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestEventNewCharRelease <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestEventNewThemeRelease <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyEventNewReleaseUpdate <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestEventFriend <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestEventBili <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamCreate <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamSearch <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamList <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamApply <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamDeclaration <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamInfo <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamPosition <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamApplyList <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamDealApply <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamKick <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamExit <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamLogs <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyTeamChange <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyTeamInfoChange <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyTeamApplyChange <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamUploadSong <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamConfirmUploadSong <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTeamBuyItem <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyTeamBuffList <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPreRankInfo <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPreRankBegin <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPreRankEnd <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPreRankRankList <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPVPBeginMatching <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPVPEndMatching <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyPVPMatchSuccess <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPVPMatchConfirm <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyPVPMatchConfirm <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyPVPStartLoading <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPVPFinishLoading <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyPVPFinishLoading <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyPVPStartGame <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPVPSyncScore <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyPVPSyncScore <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPVPUseSkill <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyPVPUseSkill <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPVPFinishGame <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::NotifyPVPFinishGame <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestPVPCurrentState <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestBuyProduct <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestVerifyIOSReceipt <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestMissingOrder <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestSendOrder <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestVerifyGooglePay <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestIOSAppReceipt <ul><li>- [ ]</li></ul></td></tr><tr><td>CometScene::RequestTestVerify <ul><li>- [ ]</li></ul></td></tr></tbody></table></td>
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
i>- [x] item1</li><li>- [ ] item2</li></ul></td></tr><tr><td>CometScene::RequestMissingOrder</td><td><ul><li>- [x] item1</li><li>- [ ] item2</li></ul></td></tr><tr><td>CometScene::RequestSendOrder</td><td><ul><li>- [x] item1</li><li>- [ ] item2</li></ul></td></tr><tr><td>CometScene::RequestVerifyGooglePay</td><td><ul><li>- [x] item1</li><li>- [ ] item2</li></ul></td></tr><tr><td>CometScene::RequestIOSAppReceipt</td><td><ul><li>- [x] item1</li><li>- [ ] item2</li></ul></td></tr><tr><td>CometScene::RequestTestVerify</td><td><ul><li>- [x] item1</li><li>- [ ] item2</li></ul></td></tr></tbody></table></td>
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
f)
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
