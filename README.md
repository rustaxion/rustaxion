# Rustaxion

## About

🚧🚧🚧🚧 <br />
A WIP reverse engineered implementation of [音灵 INVAXION](https://store.steampowered.com/app/921630/_INVAXION/)'s servers, based on the client code.

If you are looking for a working solution, try out my [server emulator](https://github.com/Invaxion-Server-Emulator/invaxion-server-emulator) implemented as a client-side mod for the game.

<!-- progress-start -->
## Progress

### Comet::Login (20% done)

| Method | Implemented |
| --------- | ------- |
| CometLogin::RequestThirdLogin | ✅ |
| CometLogin::RequestGameVersion | ✅ |
| CometLogin::RequestRegAccount | ❌ |
| CometLogin::RequestLoginAccount | ❌ |
| CometLogin::RequestFindPassword | ❌ |
| CometLogin::RequestQuickToken | ❌ |
| CometLogin::RequestQuickLogin | ❌ |
| CometLogin::RequestBindAccount | ❌ |
| CometLogin::RequestAnnouncement | ❌ |
| CometLogin::RequestBiliLogin | ❌ |

### Comet::Gate (50% done)

| Method | Implemented |
| --------- | ------- |
| CometGate::LoginGateVerify | ✅ |
| CometGate::CreateCharacter | ✅ |
| CometGate::EnterGame | ✅ |
| CometGate::NotifyGameTime | ❌ |
| CometGate::RequestUserGameTime | ❌ |
| CometGate::SelectUserInfoList | ❌ |

### Comet::Scene (3% done)

| Method | Implemented |
| --------- | ------- |
| CometScene::RequestShopInfo | ✅ |
| CometScene::RequestChangeLanguage | ✅ |
| CometScene::RequestEventInfo | ✅ |
| CometScene::NotifyCharacterFullData | ❌ |
| CometScene::RequestBeginSong | ❌ |
| CometScene::RequestFinishSong | ❌ |
| CometScene::RequestSingleSongRank | ❌ |
| CometScene::RequestRankInfo | ❌ |
| CometScene::RequestSetFavorite | ❌ |
| CometScene::RequestBackstageGame | ❌ |
| CometScene::NotifyUpdateInfo | ❌ |
| CometScene::RequestActivityInfo | ❌ |
| CometScene::RequestActivityBegin | ❌ |
| CometScene::RequestActivityFinish | ❌ |
| CometScene::NotifyActivityChange | ❌ |
| CometScene::RequestMailList | ❌ |
| CometScene::RequestGetMailReward | ❌ |
| CometScene::RequestDeleteMail | ❌ |
| CometScene::NotifyDeleteInfo | ❌ |
| CometScene::RequestGuide | ❌ |
| CometScene::RequestGuideClear | ❌ |
| CometScene::RequestChangeHeadIcon | ❌ |
| CometScene::RequestChangeCharacter | ❌ |
| CometScene::RequestChangeTheme | ❌ |
| CometScene::RequestShopBuy | ❌ |
| CometScene::RequestPieceExchange | ❌ |
| CometScene::RequestBattleFieldInfo | ❌ |
| CometScene::RequestBattleFieldRankInfo | ❌ |
| CometScene::RequestBattleFieldBegin | ❌ |
| CometScene::RequestBattleFieldFinish | ❌ |
| CometScene::RequestSummonInfo | ❌ |
| CometScene::RequestSummon | ❌ |
| CometScene::RequestSummonWeekReward | ❌ |
| CometScene::RequestSummonShopBuy | ❌ |
| CometScene::RequestSocialSearchPlayer | ❌ |
| CometScene::RequestSocialPlayerProfile | ❌ |
| CometScene::RequestSocialSendAddFriendRequest | ❌ |
| CometScene::NotifySocialAddFriendRequest | ❌ |
| CometScene::RequestSocialDeleteFriend | ❌ |
| CometScene::NotifySocialDeleteFriend | ❌ |
| CometScene::RequestSocialDisposeFriendRequest | ❌ |
| CometScene::NotifySocialDisposeFriendRequest | ❌ |
| CometScene::RequestSocialPublishDynamics | ❌ |
| CometScene::RequestSocialDeleteDynamics | ❌ |
| CometScene::RequestSocialFriendDynamics | ❌ |
| CometScene::NotifySocialFriendPublicDynamic | ❌ |
| CometScene::NotifySocialFriendStatus | ❌ |
| CometScene::RequestStoryInfo | ❌ |
| CometScene::RequestStoryFinish | ❌ |
| CometScene::RequestUseItem | ❌ |
| CometScene::RequestArcadeInfo | ❌ |
| CometScene::RequestArcadeFinish | ❌ |
| CometScene::RequestChangeTitle | ❌ |
| CometScene::RequestEventLevelGift | ❌ |
| CometScene::RequestEventStamina | ❌ |
| CometScene::RequestEventNewPlayer | ❌ |
| CometScene::RequestEventWeekCheckin | ❌ |
| CometScene::RequestEventRecharge | ❌ |
| CometScene::NotifyRechargeUpdate | ❌ |
| CometScene::RequestEventLogin | ❌ |
| CometScene::RequestEventNewCharLogin | ❌ |
| CometScene::RequestEventNewThemeLogin | ❌ |
| CometScene::RequestEventNewCharRelease | ❌ |
| CometScene::RequestEventNewThemeRelease | ❌ |
| CometScene::NotifyEventNewReleaseUpdate | ❌ |
| CometScene::RequestEventFriend | ❌ |
| CometScene::RequestEventBili | ❌ |
| CometScene::RequestTeamCreate | ❌ |
| CometScene::RequestTeamSearch | ❌ |
| CometScene::RequestTeamList | ❌ |
| CometScene::RequestTeamApply | ❌ |
| CometScene::RequestTeamDeclaration | ❌ |
| CometScene::RequestTeamInfo | ❌ |
| CometScene::RequestTeamPosition | ❌ |
| CometScene::RequestTeamApplyList | ❌ |
| CometScene::RequestTeamDealApply | ❌ |
| CometScene::RequestTeamKick | ❌ |
| CometScene::RequestTeamExit | ❌ |
| CometScene::RequestTeamLogs | ❌ |
| CometScene::NotifyTeamChange | ❌ |
| CometScene::NotifyTeamInfoChange | ❌ |
| CometScene::NotifyTeamApplyChange | ❌ |
| CometScene::RequestTeamUploadSong | ❌ |
| CometScene::RequestTeamConfirmUploadSong | ❌ |
| CometScene::RequestTeamBuyItem | ❌ |
| CometScene::NotifyTeamBuffList | ❌ |
| CometScene::RequestPreRankInfo | ❌ |
| CometScene::RequestPreRankBegin | ❌ |
| CometScene::RequestPreRankEnd | ❌ |
| CometScene::RequestPreRankRankList | ❌ |
| CometScene::RequestPVPBeginMatching | ❌ |
| CometScene::RequestPVPEndMatching | ❌ |
| CometScene::NotifyPVPMatchSuccess | ❌ |
| CometScene::RequestPVPMatchConfirm | ❌ |
| CometScene::NotifyPVPMatchConfirm | ❌ |
| CometScene::NotifyPVPStartLoading | ❌ |
| CometScene::RequestPVPFinishLoading | ❌ |
| CometScene::NotifyPVPFinishLoading | ❌ |
| CometScene::NotifyPVPStartGame | ❌ |
| CometScene::RequestPVPSyncScore | ❌ |
| CometScene::NotifyPVPSyncScore | ❌ |
| CometScene::RequestPVPUseSkill | ❌ |
| CometScene::NotifyPVPUseSkill | ❌ |
| CometScene::RequestPVPFinishGame | ❌ |
| CometScene::NotifyPVPFinishGame | ❌ |
| CometScene::RequestPVPCurrentState | ❌ |
| CometScene::RequestBuyProduct | ❌ |
| CometScene::RequestVerifyIOSReceipt | ❌ |
| CometScene::RequestMissingOrder | ❌ |
| CometScene::RequestSendOrder | ❌ |
| CometScene::RequestVerifyGooglePay | ❌ |
| CometScene::RequestIOSAppReceipt | ❌ |
| CometScene::RequestTestVerify | ❌ |

<!-- progress-end -->

## Contributing

Feel free to contribute in any way you can! <br />
I even accept [donations](https://github.com/sponsors/ArjixWasTaken) if you want to contribute but cannot code :^)

## Attributions

1. [invaxion-server](https://github.com/603185423/invaxion-server) by @MoeGrid and @603185423 for the server logic
2. [Invaxion-Server-Emulator](https://github.com/Invaxion-Server-Emulator/invaxion-server-emulator) by @ArjixWasTaken (me!) for discovering the missing parts (CosmicTour and some other stuff)
