# Rustaxion

## About

🚧🚧🚧🚧 <br />
A WIP reverse engineered implementation of [音灵 INVAXION](https://store.steampowered.com/app/921630/_INVAXION/)'s servers, based on the client code.

If you are looking for a working solution, try out my [server emulator](https://github.com/rustaxion/old-server-emulator) implemented as a client-side mod for the game.

## Community

I've created a discord server that you can hop on to share ideas or play with others!

<a href="https://discord.gg/kGQwzKNRpz" target="_blank">
    <img src="https://discord.com/api/guilds/1341538524469526604/widget.png?style=banner1" alt="Discord Banner 1"/>
</a>


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

### Comet::Gate (100% done)

| Method | Implemented |
| --------- | ------- |
| CometGate::RequestUserGameTime | ✅ |
| CometGate::ResponseUserGameTime | ✅ |
| CometGate::LoginGateVerify | ✅ |
| CometGate::CreateCharacter | ✅ |
| CometGate::EnterGame | ✅ |

### Comet::Scene (9% done)

| Method | Implemented |
| --------- | ------- |
| CometScene::RequestBeginSong | ✅ |
| CometScene::RequestFinishSong | ✅ |
| CometScene::RequestRankInfo | ✅ |
| CometScene::RequestSetFavorite | ✅ |
| CometScene::RequestShopInfo | ✅ |
| CometScene::RequestChangeLanguage | ✅ |
| CometScene::RequestSocialPublishDynamics | ✅ |
| CometScene::RequestEventInfo | ✅ |
| CometScene::RequestSingleSongRank | ❌ |
| CometScene::RequestBackstageGame | ❌ |
| CometScene::RequestActivityInfo | ❌ |
| CometScene::RequestActivityBegin | ❌ |
| CometScene::RequestActivityFinish | ❌ |
| CometScene::RequestMailList | ❌ |
| CometScene::RequestGetMailReward | ❌ |
| CometScene::RequestDeleteMail | ❌ |
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
| CometScene::RequestSocialDeleteFriend | ❌ |
| CometScene::RequestSocialDisposeFriendRequest | ❌ |
| CometScene::RequestSocialDeleteDynamics | ❌ |
| CometScene::RequestSocialFriendDynamics | ❌ |
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
| CometScene::RequestEventLogin | ❌ |
| CometScene::RequestEventNewCharLogin | ❌ |
| CometScene::RequestEventNewThemeLogin | ❌ |
| CometScene::RequestEventNewCharRelease | ❌ |
| CometScene::RequestEventNewThemeRelease | ❌ |
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
| CometScene::RequestTeamUploadSong | ❌ |
| CometScene::RequestTeamConfirmUploadSong | ❌ |
| CometScene::RequestTeamBuyItem | ❌ |
| CometScene::RequestPreRankInfo | ❌ |
| CometScene::RequestPreRankBegin | ❌ |
| CometScene::RequestPreRankEnd | ❌ |
| CometScene::RequestPreRankRankList | ❌ |
| CometScene::RequestPVPBeginMatching | ❌ |
| CometScene::RequestPVPEndMatching | ❌ |
| CometScene::RequestPVPMatchConfirm | ❌ |
| CometScene::RequestPVPFinishLoading | ❌ |
| CometScene::RequestPVPSyncScore | ❌ |
| CometScene::RequestPVPUseSkill | ❌ |
| CometScene::RequestPVPFinishGame | ❌ |
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

1. [invaxion-server](https://github.com/603185423/invaxion-server) by @MoeGrid and @603185423
2. [Invaxion-Server-Emulator](https://github.com/rustaxion/old-server-emulator) by @ArjixWasTaken (me!)
