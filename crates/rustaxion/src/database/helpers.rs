use std::collections::HashMap;

use crate::database::entities::{player, prelude::*, shop_item};

use proto::{self, comet_scene::*};
use sea_orm::{entity::*, DatabaseConnection, QueryFilter};

use super::entities::{
    player_beatmap, player_character, player_favourite_beatmap, player_theme, score,
    sea_orm_active_enums,
};

pub async fn get_announcements(_db: &DatabaseConnection) -> anyhow::Result<AnnouncementData> {
    // TODO: Populate this using data from the database.
    Ok(AnnouncementData {
        list: vec![AnnouncementOneData {
            title: "Operation Announcement".to_string(),
            content: "<b><color=#ffa500ff>《音灵INVAXION》Closing notice</color></b>\n\t\t  \n\nIt's been a long wait, guardians of the sound.\n\t\t  \nWelcome to the<color=#ffa500ff>《音灵INVAXION》</color> world.".to_string(),
            pic_id: 0,
            tag: 1,
        }],
        pic_list: vec![],
    })
}

#[rustfmt::skip]
pub async fn get_player_score_list(
    player_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<ScoreList> {
    let scores = Score::find().filter(score::Column::PlayerId.eq(player_id)).all(db).await?;
    let mut scores_cant_think_of_a_name: HashMap<(i32, i32, i32, i32), (score::Model, u32)> = std::collections::HashMap::new();

    for score in scores {
        let key = (score.beatmap_id, score.mode.clone() as i32, score.difficulty.clone() as i32, score.finish_level);
        if let Some(prev) = scores_cant_think_of_a_name.get_mut(&key) {
            prev.1 += 1;

            if prev.0.score < score.score {
                prev.0 = score;
            }
        } else {
            scores_cant_think_of_a_name.insert(key, (score, 1));
        }
    }

    let mut key4_list = DifficultyList {
        easy_list: vec![],
        normal_list: vec![],
        hard_list: vec![]
    };

    let mut key6_list = DifficultyList {
        easy_list: vec![],
        normal_list: vec![],
        hard_list: vec![]
    };

    let mut key8_list = DifficultyList {
        easy_list: vec![],
        normal_list: vec![],
        hard_list: vec![]
    };

    for (score, count) in scores_cant_think_of_a_name.values() {
        let key = match score.mode {
            sea_orm_active_enums::BeatmapMode::FourKeys => &mut key4_list,
            sea_orm_active_enums::BeatmapMode::SixKeys => &mut key6_list,
            sea_orm_active_enums::BeatmapMode::EightKeys => &mut key8_list,
        };

        let diff = match score.difficulty {
            sea_orm_active_enums::BeatmapDifficulty::Easy => &mut key.easy_list,
            sea_orm_active_enums::BeatmapDifficulty::Normal => &mut key.normal_list,
            sea_orm_active_enums::BeatmapDifficulty::Hard => &mut key.hard_list,
        };

        diff.push(SingleSongInfo {
            song_id: score.beatmap_id,
            score: score.score,
            is_full_combo: if score.is_full_combo { 1 } else { 0 },
            is_all_max: if score.is_perfect { 1 } else { 0 },
            miss: score.miss_count,
            finish_level: score.finish_level,
            play_count: *count as i32,
        });
    }

    Ok(ScoreList {
        key4_list,
        key6_list,
        key8_list,
    })
}

#[rustfmt::skip]
pub async fn get_player_song_list(
    player_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<SongList> {
    let beatmaps: Vec<SongData> = PlayerBeatmap::find()
        .filter(player_beatmap::Column::PlayerId.eq(player_id))
        .all(db).await?
        .iter()
        .map(|x| SongData { song_id: x.beatmap_id })
        .collect();

    let favourites: Vec<i32> = PlayerFavouriteBeatmap::find()
        .filter(player_favourite_beatmap::Column::PlayerId.eq(player_id))
        .all(db).await?
        .iter()
        .map(|x| x.beatmap_id)
        .collect();

    Ok(SongList {
        list: beatmaps,
        favorite_list: favourites,
    })
}

#[rustfmt::skip]
pub async fn get_player_char_list(
    player_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<CharacterList> {
    let characters = PlayerCharacter::find().filter(player_character::Column::PlayerId.eq(player_id)).all(db).await?;
    Ok(CharacterList {
        list: characters.iter().map(|x| CharData {
            char_id: x.character_id,
            level: x.level,
            exp: x.experience,
            play_count: x.play_count,
        }).collect(),
    })
}

#[rustfmt::skip]
pub async fn get_player_social_data(
    _player_id: i32,
    _db: &DatabaseConnection
) -> anyhow::Result<SocialData> {
    // TODO: Populate this using data from the database.

    Ok(SocialData {
        friend_list: vec![],
        request_list: vec![],
    })
}

#[rustfmt::skip]
pub async fn get_player_theme_list(
    player_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<ThemeList> {
    let themes = PlayerTheme::find().filter(player_theme::Column::PlayerId.eq(player_id)).all(db).await?;
    Ok(ThemeList { list: themes.iter().map(|x| ThemeData { theme_id: x.theme_id }).collect() })
}

#[rustfmt::skip]
pub async fn get_player_vip_info(
    _player_id: i32,
    _db: &DatabaseConnection
) -> anyhow::Result<PlayerVipInfo> {
    // TODO: Populate this using data from the database.

    Ok(PlayerVipInfo {
        level: 0,
        exp: 0,
        level_up_exp: 0,
        in_subscription: 0,
    })
}

#[rustfmt::skip]
pub async fn get_player_arcade_data(
    _player_id: i32,
    _db: &DatabaseConnection
) -> anyhow::Result<ArcadeData> {
    // TODO: Populate this using data from the database.

    let diff = ArcadeDiffList {
        easy_list: vec![],
        normal_list: vec![],
        hard_list: vec![]
    };
    Ok(ArcadeData {
        key4_list: diff.clone(),
        key6_list: diff.clone(),
        key8_list: diff.clone(),
    })
}

#[rustfmt::skip]
pub async fn get_player_title_data(
    _player_id: i32,
    _db: &DatabaseConnection
) -> anyhow::Result<TitleData> {
    // TODO: Populate this using data from the database.

    Ok(TitleData {
        list: vec![],
    })
}

#[rustfmt::skip]
pub async fn get_player_team(
    _player_id: i32,
    _db: &DatabaseConnection
) -> anyhow::Result<TeamData> {
    // TODO: Populate this using data from the database.

    Ok(TeamData {
        team_id: 0,
        team_name: "N/A".to_string(),
        upload_song_count: 0,
        can_upload_song: 0,
        buff_list: vec![],
    })
}

#[rustfmt::skip]
pub async fn get_player_full_data(
    player_id: i32,
    db: &DatabaseConnection
) -> anyhow::Result<CharacterFullData> {
    let player = Player::find_by_id(player_id as i32).one(db).await?;
    anyhow::ensure!(player.is_some());

    let player = player.unwrap();

    let announcement = get_announcements(db).await?;
    let base_info = player.get_base_info();
    let currency_info = player.get_currency();
    let score_list = get_player_score_list(player_id, db).await?;
    let song_list = get_player_song_list(player_id, db).await?;
    let char_list = get_player_char_list(player_id, db).await?;
    let social_data = get_player_social_data(player_id, db).await?;
    let item_list = vec![];
    let theme_list = get_player_theme_list(player_id, db).await?;
    let vip_info = get_player_vip_info(player_id, db).await?;
    let experience_list = vec![];
    let arcade_data = get_player_arcade_data(player_id, db).await?;
    let title_list = get_player_title_data(player_id, db).await?;
    let team = get_player_team(player_id, db).await?;

    Ok(CharacterFullData {
        base_info,
        currency_info,
        score_list,
        song_list,
        char_list,
        social_data,
        announcement,
        item_list,
        theme_list,
        vip_info,
        experience_list,
        arcade_data,
        title_list,
        team,
    })
}

impl player::Model {
    pub fn get_currency(&self) -> PlayerCurrencyInfo {
        PlayerCurrencyInfo {
            gold: self.gold,
            diamond: self.diamond,
            cur_stamina: self.current_stamina,
            max_stamina: self.maximum_stamina,
            honour_point: self.honour_points,
        }
    }

    pub fn get_base_info(&self) -> PlayerBaseInfo {
        PlayerBaseInfo {
            acc_id: self.account_id,
            char_id: self.id as i64,
            char_name: self.name.clone(),
            head_id: self.head_id,
            level: self.level,
            cur_exp: self.current_exp,
            max_exp: self.maximum_exp,
            guide_step: 7,
            cur_character_id: self.selected_character_id,
            cur_theme_id: self.selected_theme_id,
            online_time: 0,
            need_req_app_receipt: 0,
            active_point: 0,
            pre_rank_id: self.pre_rank,

            // TODO(arjix): Figure out what this is.
            guide_list: vec![9, 8, 7, 6, 5, 4, 3, 2, 1],

            country: self.country.into_proto() as i32,
            pre_rank_id4_k: self.pre_rank4k,
            pre_rank_id6_k: self.pre_rank6k,
            title_id: self.title_id,
        }
    }
}

impl shop_item::Model {
    pub fn into_proto(&self) -> ShopItemInfo {
        ShopItemInfo {
            id: self.item_id,
            cost_type: self.cost_type,
            normal_price: self.normal_price,
            discount_price: self.discount_price,
            order: self.order,
            begin_sale_time: self.begin_sale_time.timestamp() as u64,
            discount_begin_time: self.discount_begin_time.timestamp() as u64,
            discount_end_time: self.discount_end_time.timestamp() as u64,
        }
    }
}

#[rustfmt::skip]
impl sea_orm_active_enums::Country {
    pub fn into_proto(&self) -> proto::comet_scene::Country {
        match self {
            sea_orm_active_enums::Country::Alien => proto::comet_scene::Country::CAlien,
            sea_orm_active_enums::Country::Asean => proto::comet_scene::Country::CAsean,
            sea_orm_active_enums::Country::China => proto::comet_scene::Country::CChina,
            sea_orm_active_enums::Country::Eu => proto::comet_scene::Country::CEu,
            sea_orm_active_enums::Country::HongKong => proto::comet_scene::Country::CHongKong,
            sea_orm_active_enums::Country::Japan => proto::comet_scene::Country::CJapan,
            sea_orm_active_enums::Country::Macao => proto::comet_scene::Country::CMacao,
            sea_orm_active_enums::Country::Other => proto::comet_scene::Country::COther,
            sea_orm_active_enums::Country::SouthKorea => proto::comet_scene::Country::CSouthKorea,
            sea_orm_active_enums::Country::TaiWan => proto::comet_scene::Country::CTaiWan,
            sea_orm_active_enums::Country::UnitedKingdom => proto::comet_scene::Country::CUnitedKingdom,
            sea_orm_active_enums::Country::UnitedStates => proto::comet_scene::Country::CUnitedStates,
        }
    }

    pub fn from_proto(proto: proto::comet_scene::Country) -> Self {
        match proto {
            proto::comet_scene::Country::CAlien => sea_orm_active_enums::Country::Alien,
            proto::comet_scene::Country::CAsean => sea_orm_active_enums::Country::Asean,
            proto::comet_scene::Country::CChina => sea_orm_active_enums::Country::China,
            proto::comet_scene::Country::CEu => sea_orm_active_enums::Country::Eu,
            proto::comet_scene::Country::CHongKong => sea_orm_active_enums::Country::HongKong,
            proto::comet_scene::Country::CJapan => sea_orm_active_enums::Country::Japan,
            proto::comet_scene::Country::CMacao => sea_orm_active_enums::Country::Macao,
            proto::comet_scene::Country::COther => sea_orm_active_enums::Country::Other,
            proto::comet_scene::Country::CSouthKorea => sea_orm_active_enums::Country::SouthKorea,
            proto::comet_scene::Country::CTaiWan => sea_orm_active_enums::Country::TaiWan,
            proto::comet_scene::Country::CUnitedKingdom => sea_orm_active_enums::Country::UnitedKingdom,
            proto::comet_scene::Country::CUnitedStates => sea_orm_active_enums::Country::UnitedStates,
        }
    }
}

#[rustfmt::skip]
impl sea_orm_active_enums::Language {
    pub fn into_proto(&self) -> proto::comet_login::LanguageType {
        match self {
            sea_orm_active_enums::Language::English => proto::comet_login::LanguageType::LEnglish,
            sea_orm_active_enums::Language::China => proto::comet_login::LanguageType::LChina,
            sea_orm_active_enums::Language::Japan => proto::comet_login::LanguageType::LJapan,
            sea_orm_active_enums::Language::Traditional => proto::comet_login::LanguageType::LTraditionalChinese,
        }
    }

    pub fn from_proto(proto: proto::comet_login::LanguageType) -> Self {
        match proto {
            proto::comet_login::LanguageType::LEnglish => sea_orm_active_enums::Language::English,
            proto::comet_login::LanguageType::LChina => sea_orm_active_enums::Language::China,
            proto::comet_login::LanguageType::LJapan => sea_orm_active_enums::Language::Japan,
            proto::comet_login::LanguageType::LTraditionalChinese => sea_orm_active_enums::Language::Traditional,
        }
    }
}
