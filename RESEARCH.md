# Server-Side Research

Pseudo code for all unimplemented network handlers.
Format mirrors the existing Rust handler style used in the project.

---

## Comet::Login

### CometLogin::RequestRegAccount

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqRegAccount::decode(buffer)?;
    // req.accountName: string
    // req.mail: string
    // req.password: string (should be hashed before storing)

    // Validate: accountName not empty, mail format valid, password meets requirements
    // Check: account with req.accountName does not already exist in DB
    // If exists -> return LoginError::AccountAlreadyExists

    // Hash password (bcrypt/argon2)
    // Insert new account row: (accountName, mail, hashed_password)
    // Generate auth token + select a gate server (IP/port)

    let ret = RetRegAccount {
        data: GatewayServerData {
            gate_ip: "<gate_server_ip>".to_string(),
            gate_port: 9000,
            token: "<generated_session_token>".to_string(),
            acc_id: new_account_id,
        },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnRegAccount),
        body: ret.encode_to_vec(),
    }])
}
```

### CometLogin::RequestLoginAccount

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqLoginAccount::decode(buffer)?;
    // req.accountName: string
    // req.password: string

    // Look up account by accountName in DB
    // If not found -> return LoginError::AccountNotFound
    // Verify hashed password matches stored hash
    // If mismatch -> return LoginError::PasswordError

    // Generate session token, store it (cache/db)
    // Select a gate server

    let ret = RetLoginAccount {
        data: GatewayServerData {
            gate_ip: "<gate_server_ip>".to_string(),
            gate_port: 9000,
            token: "<generated_session_token>".to_string(),
            acc_id: account_id,
        },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnLoginAccount),
        body: ret.encode_to_vec(),
    }])
}
```

### CometLogin::RequestFindPassword

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqFindPassword::decode(buffer)?;
    // req.accountName: string
    // req.mail: string
    // req.password: string  (new password to set)

    // Look up account by accountName in DB
    // If not found -> return error
    // Verify req.mail matches the stored email on the account
    // If mismatch -> return LoginError::MailError (or similar)

    // Hash new password
    // Update account row: set hashed_password = new hash

    // Generate session token + select gate server
    let ret = RetFindPassword {
        data: GatewayServerData {
            gate_ip: "<gate_server_ip>".to_string(),
            gate_port: 9000,
            token: "<generated_session_token>".to_string(),
            acc_id: account_id,
        },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnFindPassword),
        body: ret.encode_to_vec(),
    }])
}
```

### CometLogin::RequestQuickToken

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqQuickToken::decode(buffer)?;
    // No request fields — client is asking for a guest token

    // Generate a unique guest token (UUID or random), store it in DB/cache
    // This token represents an anonymous account, not yet bound to email/password

    let ret = RetQuickToken {
        token: "<generated_guest_token>".to_string(),
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnQuickToken),
        body: ret.encode_to_vec(),
    }])
}
```

### CometLogin::RequestQuickLogin

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqQuickLogin::decode(buffer)?;
    // req.token: string  (guest token obtained from RequestQuickToken)

    // Look up guest account by token in DB/cache
    // If not found / expired -> return LoginError::TokenError

    // If first use: create a new guest account row associated with this token
    // Generate session token + select gate server

    let ret = RetQuickLogin {
        data: GatewayServerData {
            gate_ip: "<gate_server_ip>".to_string(),
            gate_port: 9000,
            token: "<generated_session_token>".to_string(),
            acc_id: guest_account_id,
        },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnQuickLogin),
        body: ret.encode_to_vec(),
    }])
}
```

### CometLogin::RequestBindAccount

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqBindAccount::decode(buffer)?;
    // req.accountName: string  (desired username)
    // req.password: string     (desired password)
    // req.token: string        (guest token identifying the account to bind)

    // Resolve guest account by req.token
    // If not found -> return error (invalid token)
    // Check req.accountName is not already taken by another account
    // If taken -> return LoginError::AccountAlreadyExists

    // Hash password
    // Update guest account row: set accountName + hashed_password + clear guest flag
    // Account is now a full account

    // Ret_BindAccount has no fields — just acknowledge success
    let ret = RetBindAccount {};

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnBindAccount),
        body: ret.encode_to_vec(),
    }])
}
```

### CometLogin::RequestAnnouncement

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqAnnouncement::decode(buffer)?;
    // No request fields

    // Load announcement from DB or config
    let ret = RetAnnouncement {
        title: "Welcome".to_string(),
        content: "Server announcement text here.".to_string(),
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnAnnouncement),
        body: ret.encode_to_vec(),
    }])
}
```

### CometLogin::RequestBiliLogin

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqBiliLogin::decode(buffer)?;
    // req.access_key: string  (Bilibili OAuth access key)
    // req.uid: uint           (Bilibili user ID)
    // req.deviceInfo: string  (device fingerprint)

    // Call Bilibili OAuth verification API with access_key + uid
    // If verification fails -> return LoginError::ThirdLoginError (or similar)

    // Look up account by (platform=Bilibili, platform_uid=req.uid)
    // If not found: auto-create account linked to this Bilibili uid

    // Generate session token + select gate server
    let ret = RetBiliLogin {
        data: GatewayServerData {
            gate_ip: "<gate_server_ip>".to_string(),
            gate_port: 9000,
            token: "<generated_session_token>".to_string(),
            acc_id: account_id,
        },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::ReturnBiliLogin),
        body: ret.encode_to_vec(),
    }])
}
```

---

## Comet::Scene

### CometScene::RequestSingleSongRank

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqSingleSongRank::decode(buffer)?;
    // req.songId: uint
    // req.difficulty: uint  (see eSongDifficulty)
    // req.mode: uint        (see eSongMode)
    // req.isWeek: uint      (0 = all-time, 1 = weekly)

    // Query score table for req.songId + req.difficulty + req.mode
    // If req.isWeek == 1: filter to scores submitted in the current week
    // Order by score DESC, limit to top N (e.g. 100)
    // For each row build SingleSongRankData {
    //   rank, charId, charName, headId, titleId, country, teamName, score, finishLevel
    // }

    let ret = RetSingleSongRank {
        list: rank_list, // Vec<SingleSongRankData>
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSingleSongRank),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestBackstageGame

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqBackstageGame::decode(buffer)?;
    // req.isBack: uint  (1 = going to background, 0 = returning to foreground)

    // Update session state: record that the player backgrounded/foregrounded
    // If isBack == 1: pause any active game session timers (e.g. activity cooldowns)
    // If isBack == 0: resume timers

    // No response body documented — this may be fire-and-forget (no Ret_ exists)
    // If a response is needed, send an empty ack

    Ok(vec![]) // or empty ack response
}
```

### CometScene::RequestActivityInfo

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqActivityInfo::decode(buffer)?;

    // Load all active activities for the player from DB
    // For each activity: load player progress (bestScore, maxCombo, curCount, missionList)
    // ActivityData fields: songId, difficulty, costType, costValue, highCostType, highCostValue,
    //   isOpen, beginTime, endTime, curCount, bestScore, maxCombo, itemList, missionList

    let ret = RetActivityInfo {
        list: activity_list, // Vec<ActivityData>
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnActivityInfo),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestActivityBegin

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqActivityBegin::decode(buffer)?;
    // req.songId: uint
    // req.isHighCost: uint  (1 = use high-cost entry, which may give better rewards)

    // Load activity config for req.songId
    // Determine cost: if req.isHighCost == 1 use highCostType/highCostValue, else costType/costValue
    // Check player has enough currency/stamina
    // Deduct cost from player inventory
    // Increment activity play count (curCount)
    // Build SettleData for the deducted cost (changeList)

    let ret = RetActivityBegin {
        data: updated_activity_data,  // ActivityData with updated curCount
        settle_data: settle,          // SettleData showing deducted cost
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnActivityBegin),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestActivityFinish

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqActivityFinish::decode(buffer)?;
    // req.songId: uint
    // req.playData: PlayData  (score, combo, etc. from the completed song)

    // Validate the play session exists and hasn't already been finished
    // Compare req.playData.score against activity bestScore — update if better
    // Compare req.playData.maxCombo against activity maxCombo — update if better
    // Evaluate mission completion (missionList) based on playData
    // Award mission rewards: add items to player inventory (build SettleData)
    // Award any rank-up rewards based on activity score thresholds

    let ret = RetActivityFinish {
        data: updated_activity_data, // ActivityData with updated stats/missions
        settle_data: settle,         // SettleData listing awarded items
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnActivityFinish),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestMailList

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqMailList::decode(buffer)?;

    // Query mail table for player's charId
    // Return all mails (read + unread), including expired ones that haven't been deleted
    // MailData fields: mailId, mailTitle, mailContent, createTime, isGet (reward claimed), rewards

    let ret = RetMailList {
        mail_list: MailList { list: mails }, // Vec<MailData>
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnMailList),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestGetMailReward

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqGetMailReward::decode(buffer)?;
    // req.mailId: ulong

    // Look up mail by req.mailId, verify it belongs to this player
    // If mail.isGet == 1 -> return error (already claimed)
    // If mail has expired -> return error
    // Grant all items in mail.rewards to player inventory
    // Set mail.isGet = 1 in DB
    // Build SettleData for granted rewards

    let ret = RetGetMailReward {
        mail_id: req.mail_id,
        settle_data: settle, // SettleData listing granted items
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnGetMailReward),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestDeleteMail

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqDelMail::decode(buffer)?;
    // No request fields — deletes all read/claimed mails

    // Delete all mails for this player where isGet == 1 (reward already claimed)
    // Or: delete all mails that have no unclaimed rewards

    let ret = RetDelMail {};

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnDeleteMail),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestGuide

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqGuide::decode(buffer)?;
    // req.step: uint  (tutorial step the player has reached)

    // Update player's tutorial progress: set guide_step = req.step in DB
    // If req.step triggers a reward (first-time milestone), grant it and add to SettleData
    // (No SettleData in Ret_Guide, so rewards are likely sent via Ntf or included elsewhere)

    let ret = RetGuide {
        step: req.step,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnGuide),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestGuideClear

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqGuideClear::decode(buffer)?;

    // Mark tutorial as fully completed for this player (guide_step = MAX or is_guide_done = true)
    // No response fields — just acknowledge

    let ret = RetGuideClear {};

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnGuideClear),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestChangeHeadIcon

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqChangeHeadIcon::decode(buffer)?;
    // req.id: uint  (head icon item ID)

    // Verify player owns the head icon with id == req.id (check inventory/unlocked list)
    // If not owned -> return error
    // Update player profile: set head_icon_id = req.id in DB

    let ret = RetChangeHeadIcon {
        id: req.id,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnChangeHeadIcon),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestChangeCharacter

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqChangeCharacter::decode(buffer)?;
    // req.id: uint  (character ID)

    // Verify player owns character req.id (check character unlock table)
    // If not owned -> return error
    // Update player profile: set active_character_id = req.id in DB

    let ret = RetChangeCharacter {
        id: req.id,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnChangeCharacter),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestChangeTheme

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqChangeTheme::decode(buffer)?;
    // req.id: uint  (theme/skin ID)

    // Verify player owns theme req.id
    // If not owned -> return error
    // Update player profile: set active_theme_id = req.id in DB

    let ret = RetChangeTheme {
        id: req.id,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnChangeTheme),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestShopBuy

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqShopBuy::decode(buffer)?;
    // req.shopType: uint  (which shop: normal, premium, etc.)
    // req.itemId: uint    (item to purchase)

    // Load shop config for req.shopType, find item with req.itemId
    // Verify item is currently available (not limited stock exhausted, within sale window)
    // Determine price (currency type + amount)
    // Check player has sufficient currency
    // Deduct price from player currency (changeList in SettleData)
    // Grant item to player inventory (updateList in SettleData)
    // Handle limited-stock decrement if applicable

    let ret = RetShopBuy {
        settle_data: settle, // SettleData with currency deducted + item granted
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnShopBuy),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestPieceExchange

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqPieceExchange::decode(buffer)?;
    // req.id: uint    (target item to obtain via exchange)
    // req.type: uint  (piece type / exchange category)
    // req.isShop: uint (1 = exchange shop variant)

    // Load exchange config for req.type + req.id
    // Calculate required pieces (cost)
    // Check player has enough pieces of req.type
    // Deduct pieces (changeList)
    // Grant target item req.id to player (updateList)

    let ret = RetPieceExchange {
        settle_data: settle,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnPieceExchange),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestBattleFieldInfo

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqBattleFieldInfo::decode(buffer)?;

    // Load all available battle fields from config
    // Load player's challenge counts: playerChallengeCount (remaining solo), teamChallengeCount
    // inRestTime: seconds until next challenge period opens (0 if open now)
    // FieldInfo per field: fieldId, songId, difficulty, required score thresholds, current top score, etc.

    let ret = RetBattleFieldInfo {
        field_list: fields,              // Vec<FieldInfo>
        player_challenge_count: 3,
        team_challenge_count: 1,
        in_rest_time: 0,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnBattleFieldInfo),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestBattleFieldRankInfo

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqBattleFieldRankInfo::decode(buffer)?;
    // req.fieldId: uint

    // Query battle field score table for req.fieldId
    // Order by score DESC, return top N entries
    // TotalSongRankData: rank, charId, charName, score, headId, titleId, teamName, country, etc.

    let ret = RetBattleFieldRankInfo {
        list: rank_list, // Vec<TotalSongRankData>
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnBattleFieldRankInfo),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestBattleFieldBegin

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqBattleFieldBegin::decode(buffer)?;
    // req.fieldId: uint

    // Verify the battle field req.fieldId is currently open
    // Check player has remaining playerChallengeCount > 0
    // Deduct 1 from playerChallengeCount (update DB)
    // Deduct entry cost if any (stamina, etc.) — record in SettleData

    let ret = RetBattleFieldBegin {
        challenge_count: remaining_count, // updated remaining challenge count
        settle_data: settle,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnBattleFieldBegin),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestBattleFieldFinish

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqBattleFieldFinish::decode(buffer)?;
    // req.fieldId: uint
    // req.list: Vec<songFinishData>  (scores per song in the field)

    // Validate play session for req.fieldId
    // Compute total score from req.list
    // Compare to player's previous best for this field — update if better
    // Compute rank among all players for req.fieldId
    // Award rank-based rewards if new rank milestone achieved (via Ntf or SettleData)

    let ret = RetBattleFieldFinish {
        field_id: req.field_id,
        score: total_score,
        rank: player_rank,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnBattleFieldFinish),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestSummonInfo

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqSummonInfo::decode(buffer)?;

    // Load summon config: prices (oneNormalPrice, oneRealPrice, fiveNormalPrice, fiveRealPrice)
    //   and stamina costs (oneStamina, fiveStamina)
    // Load the summon pool: list of SummonItemData { index, item: ItemData }
    // Load player's weekly summon progress:
    //   SummonWeekInfo { weekCount, weekGetList, rewardList: Vec<SummonWeekReward { count, rewardList }> }
    // Load player's luckyCount (pity counter)

    let ret = RetSummonInfo {
        item_list: pool,
        one_normal_price: 100,
        one_real_price: 60,
        five_normal_price: 480,
        five_real_price: 280,
        one_stamina: 60,
        five_stamina: 280,
        lucky_count: player_lucky_count,
        week_info: week_info,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSummonInfo),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestSummon

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqSummon::decode(buffer)?;
    // req.type: uint  (1 = single pull, 5 = multi-pull; or currency type)

    // Determine pull count and cost based on req.type
    // Check player has sufficient currency (normal coins or premium)
    // Deduct cost (changeList in SettleData)

    // Roll the gacha:
    //   For each pull: roll against pool weights
    //   Track luckyCount (pity system): increment per pull, reset on rare item
    //   If luckyCount reaches threshold: guarantee rare item
    // indexList: indices into the summon pool of what was obtained
    // Grant items to player inventory (updateList in SettleData)
    // Increment weekCount

    let ret = RetSummon {
        type_: req.type_,
        index_list: pulled_indices, // Vec<uint> — pool indices of obtained items
        lucky_count: new_lucky_count,
        week_count: new_week_count,
        settle_data: settle,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSummon),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestSummonWeekReward

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqSummonWeekReward::decode(buffer)?;
    // req.count: uint  (the milestone count being claimed, e.g. 10 / 30 / 50 pulls this week)

    // Verify player's weekCount >= req.count
    // Verify req.count milestone not already claimed (check weekGetList)
    // Load reward for milestone req.count from SummonWeekReward config
    // Grant reward items to player (updateList in SettleData)
    // Mark milestone req.count as claimed in weekGetList

    let ret = RetSummonWeekReward {
        count: req.count,
        settle_data: settle,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSummonWeekReward),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestSummonShopBuy

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqSummonShopBuy::decode(buffer)?;
    // req.id: uint  (summon shop item ID)

    // Load summon shop item config for req.id
    // Check stock availability and purchase limit per player
    // Determine price (summon currency / medals)
    // Check player has sufficient currency
    // Deduct cost (changeList)
    // Grant item to player (updateList)

    let ret = RetSummonShopBuy {
        id: req.id,
        settle_data: settle,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSummonShopBuy),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestSocialSearchPlayer

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqSocialSearchPlayer::decode(buffer)?;
    // req.name: string  (player name to search)

    // Query player table for exact or partial match on charName == req.name
    // If found: populate result fields
    // result: 0 = not found, 1 = found (or similar encoding)

    let ret = RetSocialSearchPlayer {
        result: 1,          // 1 = found
        char_id: found_char_id,
        name: found_name,
        head_id: head_id,
        country: country,
        is_online: is_online, // 0 or 1
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSocialSearchPlayer),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestSocialPlayerProfile

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqSocialPlayerProfile::decode(buffer)?;
    // req.charId: ulong

    // Look up player by req.charId
    // Load profile: name, level, country, isOnline
    // Load player's public dynamics (posts) — Vec<DynamicData>

    let ret = RetSocialPlayerProfile {
        data: PlayerProfileData {
            char_id: req.char_id,
            name: name,
            level: level,
            country: country,
            is_online: is_online,
            list: dynamics, // Vec<DynamicData>
        },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSocialPlayerProfile),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestSocialSendAddFriendRequest

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqSocialSendAddFriendRequest::decode(buffer)?;
    // req.charId: ulong  (target player to send friend request to)

    // Verify req.charId != self
    // Check not already friends
    // Check no existing pending request in either direction
    // Check friend list not full (self and target)
    // Insert pending friend request row (from=self, to=req.charId)
    // Notify target player if online: Ntf_Social_AddFriendRequest { from_char_id, from_name, head_id }

    let ret = RetSocialSendAddFriendRequest {
        char_id: req.char_id, // echoes target charId (confirms request was sent)
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSocialSendAddFriendRequest),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestSocialDeleteFriend

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqSocialDeleteFriend::decode(buffer)?;
    // req.charId: ulong  (friend to remove)

    // Verify friendship exists between self and req.charId
    // Delete friendship rows in both directions from friend table
    // Notify req.charId if online: Ntf_Social_DelFriend { char_id: self }

    let ret = RetSocialDeleteFriend {
        char_id: req.char_id,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSocialDeleteFriend),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestSocialDisposeFriendRequest

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqSocialDisposeFriendRequest::decode(buffer)?;
    // req.charId: ulong   (the player who sent the request)
    // req.isAccept: uint  (1 = accept, 0 = decline)

    // Find pending friend request from req.charId to self
    // If not found -> return error
    // Delete the pending request row
    // If req.isAccept == 1:
    //   Insert friendship rows (self <-> req.charId) in friend table
    //   Notify req.charId if online: Ntf_Social_DisposeFriendRequest { char_id: self, accepted: 1 }
    // If req.isAccept == 0:
    //   Notify req.charId if online: declined

    let ret = RetSocialDisposeFriendRequest {
        char_id: req.char_id,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSocialDisposeFriendRequest),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestSocialDeleteDynamics

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqSocialDeleteDynamics::decode(buffer)?;
    // req.index: uint  (index/ID of the dynamic/post to delete)

    // Verify the dynamic with req.index belongs to this player
    // Delete the dynamic row from DB

    let ret = RetSocialDeleteDynamics {
        index: req.index,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSocialDeleteDynamics),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestSocialFriendDynamics

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqSocialFriendDynamics::decode(buffer)?;

    // Load player's friend list
    // For each friend: load their recent dynamics (DynamicData: index, content, time, isShare, like)
    // Build Vec<FriendDynamics> where each entry has { friendId, list: Vec<DynamicData> }

    let ret = RetSocialFriendDynamics {
        list: friend_dynamics, // Vec<FriendDynamics>
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSocialFriendDynamics),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestStoryInfo

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqStoryInfo::decode(buffer)?;

    // Load player's story progress from DB
    // curNormalChapterId / curNormalLevelId: furthest unlocked normal chapter/level
    // curTutorialChapterId / curTutorialLevelId: tutorial progress
    // list: Vec<StoryData> — per-level data { chapterId, levelId, curRank, finishLevel, maxScore, maxCombo, missionList }
    // specialList: Vec<SpecialStoryData> — special/event story progress

    let ret = RetStoryInfo {
        cur_normal_chapter_id: chapter_id,
        cur_normal_level_id: level_id,
        cur_tutorial_chapter_id: tut_chapter,
        cur_tutorial_level_id: tut_level,
        list: story_list,
        special_list: special_list,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnStoryInfo),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestStoryFinish

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqStoryFinish::decode(buffer)?;
    // req.data: StoryFinishData  (chapterId, levelId, score, combo, rank, etc.)

    // Validate the story level is unlocked for this player
    // Update story progress:
    //   Update StoryData for this levelId: maxScore, maxCombo, curRank, missionList completions
    //   If new level cleared: advance curNormalLevelId / curNormalChapterId
    // Award rewards for first-clear missions
    // Check if next chapter unlocked

    let ret = RetStoryFinish {
        cur_normal_chapter_id: updated_chapter,
        cur_normal_level_id: updated_level,
        cur_special_chapter_id: special_chapter,
        cur_special_level_id: special_level,
        cur_tutorial_chapter_id: tut_chapter,
        cur_tutorial_level_id: tut_level,
        data: updated_story_data, // StoryData with updated stats
        settle_data: settle,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnStoryFinish),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestUseItem

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqUseItem::decode(buffer)?;
    // req.id: uint  (item ID to use, e.g. exp potion, stamina refill)

    // Look up item req.id in player inventory, verify count >= 1
    // Determine item effect based on eItemType
    // Apply effect:
    //   EXP item: grant experience, check level up (update ExperienceInfo)
    //   Stamina item: restore stamina
    //   Other consumable: apply appropriate effect
    // Deduct 1 from item count in inventory (changeList)
    // Record updates in SettleData (updateList for modified stats)

    let ret = RetUseItem {
        id: req.id,
        count: remaining_count, // remaining count of this item
        experience: exp_info,   // ExperienceInfo if exp was gained
        settle_data: settle,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnUseItem),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestArcadeInfo

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqArcadeInfo::decode(buffer)?;

    // Load arcade stage config and player's arcade progress
    // ArcadeStageData: stageId, songList, unlockCondition, player's clear status per song
    // Songs have difficulty variants (key4List, key6List, key8List via ArcadeDiffList)

    let ret = RetArcadeInfo {
        stage_list: stages, // Vec<ArcadeStageData>
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnArcadeInfo),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestArcadeFinish

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqArcadeFinish::decode(buffer)?;
    // req.finishList: Vec<ArcadeFinishData>  (one entry per song played in the arcade run)

    // Validate each ArcadeFinishData (songId, score, etc.)
    // Update player's arcade progress for each song: bestScore, rank
    // Check if stage completion condition met -> unlock next stage
    // Award stage-clear rewards if first clear
    // Build SettleData with granted rewards

    let ret = RetArcadeFinish {
        settle_data: settle,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnArcadeFinish),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestChangeTitle

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqChangeTitle::decode(buffer)?;
    // req.titleId: uint

    // Verify player owns title req.titleId (check title unlock table)
    // If not owned -> return error
    // Update player profile: set active_title_id = req.titleId in DB

    let ret = RetChangeTitle {
        title_id: req.title_id,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnChangeTitle),
        body: ret.encode_to_vec(),
    }])
}
```

---

### Event Handlers (all share Ret_Event_GetCommon)

All event claim handlers follow the same pattern:
- Decode request (which identifies what to claim via `index` or `level`/`day`)
- Verify the event is currently active (within beginTime..endTime)
- Verify the specific reward hasn't already been claimed
- Grant reward items to player
- Mark as claimed
- Return `RetEventGetCommon { index, settle_data }`

```rust
// Ret_Event_GetCommon shape:
// RetEventGetCommon { index: uint, settle_data: SettleData }
```

### CometScene::RequestEventLevelGift

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqEventLevelGift::decode(buffer)?;
    // req.level: uint  (player level milestone to claim reward for)

    // Load active level-gift event config
    // Verify player's current level >= req.level
    // Verify reward for req.level not yet claimed (check claimed set)
    // Grant reward items to player inventory
    // Mark req.level as claimed

    let ret = RetEventGetCommon { index: req.level, settle_data: settle };
    Ok(vec![Response { main_cmd: MainCmd::Scene, para_cmd: ParaCmd::CometScene(CometScene::ReturnEventLevelGift), body: ret.encode_to_vec() }])
}
```

### CometScene::RequestEventStamina

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqEventStamina::decode(buffer)?;
    // No fields — claim the daily free stamina event reward

    // Load active stamina event config
    // Verify player hasn't claimed today's stamina event reward
    // Grant stamina items
    // Mark today as claimed

    let ret = RetEventGetCommon { index: 0, settle_data: settle };
    Ok(vec![Response { main_cmd: MainCmd::Scene, para_cmd: ParaCmd::CometScene(CometScene::ReturnEventStamina), body: ret.encode_to_vec() }])
}
```

### CometScene::RequestEventNewPlayer

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqEventNewPlayer::decode(buffer)?;
    // req.day: uint  (day index of new-player event reward to claim, e.g. day 1..7)

    // Load new-player event config
    // Verify player's account age >= req.day (or simply that req.day <= today's login day)
    // Verify reward for req.day not yet claimed
    // Grant reward
    // Mark claimed

    let ret = RetEventGetCommon { index: req.day, settle_data: settle };
    Ok(vec![Response { main_cmd: MainCmd::Scene, para_cmd: ParaCmd::CometScene(CometScene::ReturnEventNewPlayer), body: ret.encode_to_vec() }])
}
```

### CometScene::RequestEventWeekCheckin

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqEventWeekCheckin::decode(buffer)?;
    // req.day: uint  (day index in the weekly check-in calendar, 1..7)

    // Load weekly check-in event config
    // Verify current weekday matches req.day (or day is unlocked)
    // Verify req.day not yet claimed this week
    // Grant reward for req.day
    // Mark req.day claimed in WeekCheckinData

    let ret = RetEventGetCommon { index: req.day, settle_data: settle };
    Ok(vec![Response { main_cmd: MainCmd::Scene, para_cmd: ParaCmd::CometScene(CometScene::ReturnEventWeekCheckin), body: ret.encode_to_vec() }])
}
```

### CometScene::RequestEventRecharge

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqEventRecharge::decode(buffer)?;
    // req.index: uint  (recharge milestone index to claim)

    // Load active recharge event config
    // Verify player's total recharge amount >= milestone threshold for req.index
    // Verify req.index milestone not yet claimed
    // Grant reward
    // Mark claimed

    let ret = RetEventGetCommon { index: req.index, settle_data: settle };
    Ok(vec![Response { main_cmd: MainCmd::Scene, para_cmd: ParaCmd::CometScene(CometScene::ReturnEventRecharge), body: ret.encode_to_vec() }])
}
```

### CometScene::RequestEventLogin

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqEventLogin::decode(buffer)?;
    // req.index: uint  (login day index within the event period)

    // Load active login event config
    // Verify player has logged in on day req.index of the event
    // Verify not yet claimed
    // Grant reward
    // Mark claimed

    let ret = RetEventGetCommon { index: req.index, settle_data: settle };
    Ok(vec![Response { main_cmd: MainCmd::Scene, para_cmd: ParaCmd::CometScene(CometScene::ReturnEventLogin), body: ret.encode_to_vec() }])
}
```

### CometScene::RequestEventNewCharLogin

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqEventNewCharLogin::decode(buffer)?;
    // req.index: uint  (login day index for a new character's login event)

    // Load new-character login event config
    // Verify player has completed req.index login days since the character was released
    // Verify not yet claimed
    // Grant reward

    let ret = RetEventGetCommon { index: req.index, settle_data: settle };
    Ok(vec![Response { main_cmd: MainCmd::Scene, para_cmd: ParaCmd::CometScene(CometScene::ReturnEventNewCharLogin), body: ret.encode_to_vec() }])
}
```

### CometScene::RequestEventNewThemeLogin

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqEventNewThemeLogin::decode(buffer)?;
    // req.index: uint  (login day index for a new theme's login event)

    // Same pattern as NewCharLogin but for a newly released theme/skin

    let ret = RetEventGetCommon { index: req.index, settle_data: settle };
    Ok(vec![Response { main_cmd: MainCmd::Scene, para_cmd: ParaCmd::CometScene(CometScene::ReturnEventNewThemeLogin), body: ret.encode_to_vec() }])
}
```

### CometScene::RequestEventNewCharRelease

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqEventNewCharRelease::decode(buffer)?;
    // req.index: uint  (release reward index for a new character)

    // Load new-character release event config
    // Verify player meets the condition for req.index (e.g. owns the character, or played N songs)
    // Verify not yet claimed
    // Grant reward

    let ret = RetEventGetCommon { index: req.index, settle_data: settle };
    Ok(vec![Response { main_cmd: MainCmd::Scene, para_cmd: ParaCmd::CometScene(CometScene::ReturnEventNewCharRelease), body: ret.encode_to_vec() }])
}
```

### CometScene::RequestEventNewThemeRelease

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqEventNewThemeRelease::decode(buffer)?;
    // req.index: uint  (release reward index for a new theme)

    // Same pattern as NewCharRelease but for a newly released theme

    let ret = RetEventGetCommon { index: req.index, settle_data: settle };
    Ok(vec![Response { main_cmd: MainCmd::Scene, para_cmd: ParaCmd::CometScene(CometScene::ReturnEventNewThemeRelease), body: ret.encode_to_vec() }])
}
```

### CometScene::RequestEventFriend

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqEventFriend::decode(buffer)?;
    // req.index: uint  (friend count milestone index, e.g. 1 friend, 5 friends, 10 friends)

    // Load active friend event config
    // Count player's current friends
    // Verify friend count >= milestone threshold for req.index
    // Verify req.index milestone not yet claimed
    // Grant reward

    let ret = RetEventGetCommon { index: req.index, settle_data: settle };
    Ok(vec![Response { main_cmd: MainCmd::Scene, para_cmd: ParaCmd::CometScene(CometScene::ReturnEventFriend), body: ret.encode_to_vec() }])
}
```

### CometScene::RequestEventBili

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqEventBili::decode(buffer)?;
    // req.index: uint  (Bilibili platform-specific event reward index)

    // Load active Bili event config
    // Verify player account is linked to Bilibili
    // Verify condition for req.index is met (e.g. specific Bili task completed)
    // Verify not yet claimed
    // Grant reward

    let ret = RetEventGetCommon { index: req.index, settle_data: settle };
    Ok(vec![Response { main_cmd: MainCmd::Scene, para_cmd: ParaCmd::CometScene(CometScene::ReturnEventBili), body: ret.encode_to_vec() }])
}
```

---

### Team Handlers

### CometScene::RequestTeamCreate

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqTeamCreate::decode(buffer)?;
    // req.teamName: string
    // req.teamDeclaration: string

    // Verify player is not already in a team
    // Verify req.teamName is not taken and meets length/content rules
    // Check cooldown: player must wait leftTime seconds before creating again (if recently left)
    // If on cooldown: return RetTeamCreate { isSuccess: 0, leftTime: remaining_seconds, info: empty }

    // Create new team row: (teamName, declaration, leaderId=self, createTime=now)
    // Add player as first member with position = Leader

    let ret = RetTeamCreate {
        is_success: 1,
        left_time: 0,
        info: full_team_info, // TeamInfoData { baseInfo, memberList, uploadSong, shop }
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamCreate),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamSearch

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqTeamSearch::decode(buffer)?;
    // req.name: string  (team name to search)

    // Query team table for team with name == req.name (exact match)
    // isFind: 1 if found, 0 if not

    let ret = RetTeamSearch {
        is_find: 1,
        info: TeamBaseInfo { team_id, team_name, leader_id, leader_name, member_count, rank, score, declaration, create_time, apply_count },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamSearch),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamList

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqTeamList::decode(buffer)?;

    // Load ranked team list (by score DESC), return top N teams
    // TeamListData: teamId, teamName, memberCount, rank
    // Also load player's current pending apply list (teamIds they've applied to)

    let ret = RetTeamList {
        list: team_list,    // Vec<TeamListData>
        apply_list: vec![], // Vec<uint> — teamIds player has pending applications for
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamList),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamApply

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqTeamApply::decode(buffer)?;
    // req.teamId: uint
    // req.isCancel: uint  (1 = cancel existing application, 0 = new application)

    // If req.isCancel == 1:
    //   Delete pending application from self to req.teamId
    //   Return RetTeamApply { isSuccess: 1, leftTime: 0 }
    // Else:
    //   Verify player not already in a team
    //   Verify team req.teamId exists and has room (memberCount < max)
    //   Check application cooldown
    //   Insert application row
    //   Notify team leader/officers: Ntf_Team_ApplyChange

    let ret = RetTeamApply {
        is_success: 1,
        left_time: 0, // cooldown seconds if on cooldown
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamApply),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamDeclaration

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqTeamDeclaration::decode(buffer)?;
    // req.teamDeclaration: string

    // Verify player is in a team and has permission (leader or officer)
    // Validate declaration length/content
    // Update team.declaration = req.teamDeclaration in DB
    // Broadcast Ntf_Team_InfoChange to all online team members

    let ret = RetTeamDeclaration {};

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamDeclaration),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamInfo

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqTeamInfo::decode(buffer)?;

    // Verify player is in a team
    // Load full TeamInfoData:
    //   baseInfo: TeamBaseInfo (name, leader, memberCount, rank, score, declaration)
    //   memberList: Vec<TeamMemberData>
    //   uploadSong: TeamUploadSongData
    //   shop: TeamShopData

    let ret = RetTeamInfo {
        info: full_team_info,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamInfo),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamPosition

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqTeamPosition::decode(buffer)?;
    // req.memberId: ulong   (target member to change position of)
    // req.position: uint    (new position/rank, see eTeamPosition)

    // Verify requesting player is team leader
    // Verify req.memberId is in the same team
    // Validate req.position is a valid eTeamPosition value
    // Update team_member.position = req.position for req.memberId in DB
    // Notify all online team members: Ntf_Team_Change

    let ret = RetTeamPosition {};

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamPosition),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamApplyList

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqTeamApplyList::decode(buffer)?;

    // Verify player is team leader or officer
    // Load all pending application rows for player's team
    // TeamApplyData: charId, charName, level, headId, etc.

    let ret = RetTeamApplyList {
        list: apply_list, // Vec<TeamApplyData>
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamApplyList),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamDealApply

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqTeamDealApply::decode(buffer)?;
    // req.applyCharId: ulong  (applicant)
    // req.isAccept: uint      (1 = accept, 0 = reject)

    // Verify requesting player is leader or officer
    // Find pending application from req.applyCharId
    // Delete the application row
    // If req.isAccept == 1:
    //   Check team not full
    //   Insert team_member row for req.applyCharId with position = Member
    //   Notify req.applyCharId if online: Ntf_Team_Change (accepted)
    //   Return newMember: TeamMemberData

    let ret = RetTeamDealApply {
        apply_char_id: req.apply_char_id,
        member_count: updated_count,
        new_member: new_member_data, // TeamMemberData (only populated if accepted)
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamDealApply),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamKick

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqTeamKick::decode(buffer)?;
    // req.memberId: ulong  (member to kick)

    // Verify requesting player is team leader
    // Verify req.memberId is in the team and is not the leader
    // Remove req.memberId from team_member table
    // Notify req.memberId if online: Ntf_Team_Change (kicked)
    // Log the event in team logs (eTeamLog::Kick)

    let ret = RetTeamKick {
        member_id: req.member_id,
        member_count: updated_count,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamKick),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamExit

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqTeamExit::decode(buffer)?;

    // Verify player is in a team
    // If player is leader and team has other members -> return error (must transfer leadership first)
    // If player is leader and is the only member -> disband team (delete all rows)
    // Else: remove player from team_member table
    // Set leave cooldown timestamp on player
    // Log the event (eTeamLog::Exit)

    let ret = RetTeamExit {};

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamExit),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamLogs

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqTeamLogs::decode(buffer)?;

    // Verify player is in a team
    // Load recent team log entries (last N, ordered by time DESC)
    // TeamLogOneData: logType (eTeamLog), charId, charName, time, extra info

    let ret = RetTeamLogs {
        log_list: logs, // Vec<TeamLogOneData>
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamLogs),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamUploadSong

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqTeamUploadSong::decode(buffer)?;
    // req.data: TeamUploadSongOneData  (songId, difficulty, score, etc.)

    // Verify player is in a team with canUploadSong == 1
    // Verify team's uploadSongCount < max allowed submissions
    // Validate the song/score data (must match a real completed play session)
    // Store submission in team_upload_song table (pending confirmation by leader)
    // Increment uploadSongCount

    let ret = RetTeamUploadSong {
        upload_song_count: new_count,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamUploadSong),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamConfirmUploadSong

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqTeamConfirmUploadSong::decode(buffer)?;
    // req.list: Vec<TeamUploadSongOneData>  (songs the leader selects to finalize/submit)

    // Verify requesting player is team leader
    // For each entry in req.list: validate it's in the pending upload list
    // Finalize the selected songs as the team's official submission for this period
    // Clear pending upload list
    // Notify all team members: Ntf_Team_Change

    let ret = RetTeamConfirmUploadSong {};

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamConfirmUploadSong),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTeamBuyItem

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqTeamBuyItem::decode(buffer)?;
    // req.type: uint  (shop category)
    // req.id: uint    (item ID in team shop)

    // Verify player is in a team
    // Load team shop item config for req.type + req.id
    // Check item availability (stock, player purchase limit)
    // Determine cost (team currency or individual currency)
    // Check player has sufficient currency
    // Deduct cost, grant item
    // Update TeamShopData for this team (stock, purchase counts)

    let ret = RetTeamBuyItem {
        shop: updated_shop_data, // TeamShopData with updated availability
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTeamBuyItem),
        body: ret.encode_to_vec(),
    }])
}
```

---

### PreRank Handlers

### CometScene::RequestPreRankInfo

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqPreRankInfo::decode(buffer)?;

    // Load player's pre-rank (ranked mode) progress for each key mode:
    //   preRank (default), preRank4K, preRank6K
    // Each PreRankData: { curRank, list: Vec<PreRankSingleLevelData> }
    // PreRankSingleLevelData: levelId, score, rank, etc.

    let ret = RetPreRankInfo {
        pre_rank: rank_data,
        pre_rank_4k: rank_4k,
        pre_rank_6k: rank_6k,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnPreRankInfo),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestPreRankBegin

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqPreRankBegin::decode(buffer)?;
    // req.type: uint    (key mode: 4K, 6K, default)
    // req.levelId: uint (the ranked level to play)

    // Verify levelId is unlocked for the player in this rank type
    // Deduct entry cost (stamina or ranked tickets) if applicable
    // Record a pending ranked session (type, levelId, started_at)

    let ret = RetPreRankBegin {
        type_: req.type_,
        level_id: req.level_id,
        settle_data: settle, // SettleData for entry cost deducted
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnPreRankBegin),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestPreRankEnd

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqPreRankEnd::decode(buffer)?;
    // req.type: uint
    // req.data: PreRankSingleLevelData  (score result for the level)

    // Validate the pending session exists for this type + levelId
    // Update player's best score for this ranked level if improved
    // Recalculate player's curRank based on updated scores
    // Award rank-up rewards if curRank improved (add to SettleData)
    // openData: the updated PreRankSingleLevelData (may unlock next level)

    let ret = RetPreRankEnd {
        type_: req.type_,
        new_rank: updated_cur_rank,
        open_data: updated_level_data,
        settle_data: settle,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnPreRankEnd),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestPreRankRankList

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqPreRankRankList::decode(buffer)?;
    // req.type: uint
    // req.levelId: uint

    // Query ranked score table for req.type + req.levelId
    // Order by score DESC, return top N
    // PreRankListData: charId, charName, headId, score, rank, etc.

    let ret = RetPreRankRankList {
        type_: req.type_,
        level_id: req.level_id,
        list: rank_list, // Vec<PreRankListData>
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnPreRankRankList),
        body: ret.encode_to_vec(),
    }])
}
```

---

### PVP Handlers

PVP uses a matchmaking system with rooms. Several operations broadcast notifications to all players in the room.

### CometScene::RequestPVPBeginMatching

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqPVPBeginMatching::decode(buffer)?;

    // Add player to the matchmaking queue (rank-based matchmaking pool)
    // Start or join a match search (async background task)
    // When a match is found for 2+ players: create a room, send Ntf_PVP_MatchSuccess to all matched players:
    //   Ntf_PVP_MatchSuccess { roomId, list: Vec<PVPPlayerInfo> }
    // The Ret_ is empty — just acknowledges the player entered the queue

    let ret = RetPVPBeginMatching {};

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnPVPBeginMatching),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestPVPEndMatching

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqPVPEndMatching::decode(buffer)?;

    // Remove player from the matchmaking queue (cancelled search)

    let ret = RetPVPEndMatching {};

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnPVPEndMatching),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestPVPMatchConfirm

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqPVPMatchConfim::decode(buffer)?;
    // req.roomId: ulong  (room to confirm joining)

    // Mark player as "confirmed" in the PVP room
    // If all players in the room have confirmed:
    //   Select a random song from ranked pool
    //   Transition room state to "loading"
    //   Send Ntf_PVP_StartLoading to all players in the room
    // If confirm window expires and not all confirmed:
    //   Cancel match, return unconfirmed players to queue

    let ret = RetPVPMatchConfim {};

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnPVPMatchConfirm),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestPVPFinishLoading

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqPVPFinishLoading::decode(buffer)?;
    // req.roomId: ulong

    // Mark player as "finished loading" in the room
    // If all players finished loading (or timeout reached):
    //   Transition room to "in game" state
    //   Send Ntf_PVP_StartGame { songId } to all players
    //   (No Ret_ — server pushes Ntf_PVP_FinishLoading to other players if needed)

    Ok(vec![]) // ack only, game start sent via Ntf
}
```

### CometScene::RequestPVPSyncScore

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqPVPSyncScore::decode(buffer)?;
    // req.roomId: ulong
    // req.score: uint  (player's current score mid-song)

    // Update player's current score in the room state
    // Broadcast Ntf_PVP_SyncScore { list: Vec<PVPScoreInfo { charId, score }> }
    //   to all OTHER players in the room

    Ok(vec![]) // no direct response; broadcast only
}
```

### CometScene::RequestPVPUseSkill

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqPVPUseSkill::decode(buffer)?;
    // req.roomId: ulong

    // Validate player is in the room and skill is not on cooldown
    // Apply skill effect to room state
    // Broadcast Ntf_PVP_UseSkill {} to all OTHER players in the room
    //   (client interprets skill effect based on game logic)

    Ok(vec![]) // broadcast only
}
```

### CometScene::RequestPVPFinishGame

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqPVPFinishGame::decode(buffer)?;
    // req.roomId: ulong

    // Mark player as "finished" in the room
    // When all players finished (or timeout):
    //   Determine winner by final scores
    //   Award winner/loser rewards (PVP rank points, items)
    //   Broadcast Ntf_PVP_FinishGame {} to all players
    //   Clean up room state

    Ok(vec![]) // results sent via Ntf_PVP_FinishGame
}
```

### CometScene::RequestPVPCurrentState

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqPVPCurState::decode(buffer)?;
    // req.roomId: ulong  (client requests current room state, e.g. after reconnect)

    // Look up room by req.roomId
    // Return current state integer:
    //   0 = waiting for confirm, 1 = loading, 2 = in game, 3 = finished
    // (Client uses this to re-sync after disconnect)

    let ret = RetPVPCurState {
        state: room_state, // uint
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnPVPCurrentState),
        body: ret.encode_to_vec(),
    }])
}
```

---

### Payment Handlers

### CometScene::RequestBuyProduct

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqBuyProduct::decode(buffer)?;
    // req.productId: string  (IAP product ID from the store listing)

    // Look up product config for req.productId
    // Create an order row: (orderId, charId, productId, status=pending, createdAt=now)
    // Generate a payment request signature
    // Return the order details to the client so it can initiate the store payment flow
    // notifyURL: webhook URL the payment processor should call on completion

    let ret = RetBuyProduct {
        order_id: new_order_id,
        sign: "<payment_signature>".to_string(),
        notify_url: "<payment_callback_url>".to_string(),
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnBuyProduct),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestVerifyIOSReceipt

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqVerifyIOSReceipt::decode(buffer)?;
    // req.orderId: uint
    // req.receipt: string  (Apple receipt data, base64)

    // Look up order req.orderId, verify it belongs to this player and is pending
    // Send req.receipt to Apple receipt verification API
    // If Apple returns status != 0 -> return error / set status to failed
    // If valid:
    //   Mark order as complete
    //   Grant purchased items/currency to player (via Ntf_RechangeUpdate or similar)
    // status: 0 = success, other = error code from Apple

    let ret = RetVerifyIOSReceipt {
        status: 0,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnVerifyIOSReceipt),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestMissingOrder

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let _req = ReqMissingOrder::decode(buffer)?;

    // Query order table for this player: find orders that are completed (payment verified)
    //   but items have not yet been delivered (status = paid, not fulfilled)
    // Return list of such orderIds — client will call RequestSendOrder for each

    let ret = RetMissingOrder {
        order_list: unfulfilled_order_ids, // Vec<uint>
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnMissingOrder),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestSendOrder

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqSendOrder::decode(buffer)?;
    // req.orderId: uint

    // Look up order req.orderId: verify it belongs to player and is in paid/verified state
    // If already fulfilled -> return error (or idempotently re-return the settle data)
    // Grant product items/currency to player inventory
    // Update VIP level if applicable (PlayerVIPInfo)
    // Mark order as fulfilled
    // Send Ntf_RechangeUpdate to client

    let ret = RetSendOrder {
        settle_data: settle,
        vip_info: PlayerVIPInfo { /* updated VIP level, exp, rewards */ },
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnSendOrder),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestVerifyGooglePay

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqVerifyGooglePay::decode(buffer)?;
    // req.orderId: uint
    // req.purchaseJson: string  (Google Play purchase token JSON)

    // Look up order req.orderId
    // Call Google Play Developer API to verify the purchase token in req.purchaseJson
    // If verification fails -> return error
    // If valid:
    //   Acknowledge the purchase with Google (to consume/acknowledge the token)
    //   Mark order as complete

    let ret = RetVerifyGooglePay {
        status: 0, // 0 = success
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnVerifyGooglePay),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestIOSAppReceipt

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqIOSAppReceipt::decode(buffer)?;
    // req.receipt: string  (full iOS app receipt, base64)

    // Send req.receipt to Apple's receipt verification endpoint
    // Parse the response to check in-app subscription status
    // inSubscription: 1 if player has an active subscription, 0 otherwise

    let ret = RetIOSAppReceipt {
        in_subscription: subscription_active, // uint: 0 or 1
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnIOSAppReceipt),
        body: ret.encode_to_vec(),
    }])
}
```

### CometScene::RequestTestVerify

```rust
pub async fn handle(session, db, buffer) -> Result<Vec<Response>> {
    let req = ReqTestVerify::decode(buffer)?;
    // req.orderId: uint  (test/sandbox order to fulfill without real payment)

    // Only allow in non-production environments (check server config flag)
    // If production -> return error
    // Look up test order req.orderId (or create a fake one)
    // Mark as fulfilled, grant product to player (same logic as RequestSendOrder)

    let ret = RetTestVerify {
        order_id: req.order_id,
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Scene,
        para_cmd: ParaCmd::CometScene(CometScene::ReturnTestVerify),
        body: ret.encode_to_vec(),
    }])
}
```
