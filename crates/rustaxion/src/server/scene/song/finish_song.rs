use anyhow::Context;
use prost::Message;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, Related,
    Set, Unchanged,
};

use crate::{
    database::{
        entities::{
            self, character, player, player_character,
            prelude::{self, Player, Score},
            score,
            sea_orm_active_enums::*,
        },
        helpers::get_player_full_data,
    },
    types::{response::Response, session::SessionData},
};

use proto::comet_scene::{PlayerExpData, ReqFinishSong, RetFinishSong, SettleData, SettleItemData};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

// TODO: Use sea-ql transactions, to prevent incomplete updates
pub async fn handle(
    session: &mut SessionData,
    db: sea_orm::DatabaseConnection,
    body: Vec<u8>,
) -> anyhow::Result<Vec<Response>> {
    let now = chrono::Utc::now().fixed_offset();
    let req = ReqFinishSong::decode(body.as_slice()).context("Failed to decode ReqFinishSong.")?;

    anyhow::ensure!(session.now_playing.is_some());
    let now_playing = session.now_playing.as_ref().unwrap().clone();
    let song_data = req.data;

    anyhow::ensure!(now_playing.song_id == song_data.song_id);
    anyhow::ensure!(now_playing.mode == song_data.mode);

    session.now_playing = None;

    let player = Player::find_by_id(session.player_id.unwrap())
        .one(&db)
        .await?;
    anyhow::ensure!(player.is_some());
    let mut player = player.unwrap();

    let play_data = song_data.play_data;

    let mut reward_exp: u32 = 0;
    if play_data.max_percent == 100 {
        reward_exp += f32::floor((player.current_exp as f32) * 0.15) as u32;
    }

    let mode = match song_data.mode {
        1 => BeatmapMode::FourKeys,
        2 => BeatmapMode::SixKeys,
        3 => BeatmapMode::EightKeys,
        _ => anyhow::bail!("Invalid mode"),
    };

    let diff = match song_data.difficulty {
        1 => BeatmapDifficulty::Easy,
        2 => BeatmapDifficulty::Normal,
        3 => BeatmapDifficulty::Hard,
        _ => anyhow::bail!("Invalid difficulty"),
    };

    match mode {
        BeatmapMode::FourKeys => reward_exp += 5,
        BeatmapMode::SixKeys => reward_exp += 7,
        BeatmapMode::EightKeys => reward_exp += 15,
    }

    match diff {
        BeatmapDifficulty::Easy => reward_exp += 5,
        BeatmapDifficulty::Normal => reward_exp += 10,
        BeatmapDifficulty::Hard => reward_exp += 15,
    }

    let scores = Score::find()
        .filter(score::Column::PlayerId.eq(session.player_id.unwrap()))
        .filter(score::Column::BeatmapId.eq(song_data.song_id))
        .filter(score::Column::Mode.eq(mode.clone()))
        .filter(score::Column::Difficulty.eq(diff.clone()))
        .all(&db)
        .await?;

    let high_score = scores.iter().max_by_key(|s| s.score);

    let mut reward_diamonds = 0;
    let mut reward_character_exp = 1;

    match high_score {
        Some(old_score) => {
            if play_data.score > old_score.score {
                reward_exp += f32::floor((reward_exp as f32) * 0.8f32) as u32;
            }
        }
        None => {
            if play_data.accuracy > 50 {
                // First clear of the chart with accuracy > 50%
                // so we reward the player with 100 diamonds
                reward_diamonds += 100;

                // and we also give some good exp to the character
                reward_character_exp += 100;
            }
        }
    }

    if play_data.is_full_combo == 1 {
        reward_exp += f32::floor((reward_exp as f32) * 0.3f32) as u32;
    }

    if play_data.is_all_max == 1 {
        reward_exp += f32::floor((reward_exp as f32) * 0.5f32) as u32;
    }

    // If the player failed to clear the chart, we reduce the rewards by half
    if play_data.life <= 0 {
        reward_exp /= 2;
    }

    let character = prelude::PlayerCharacter::find_by_id((player.id, player.selected_character_id))
        .one(&db)
        .await?;
    anyhow::ensure!(character.is_some());
    let mut character = character.unwrap();

    character.play_count += 1;
    character.experience += reward_character_exp;

    while character.experience >= 30 {
        character.experience -= 30;
        character.level += 1;
    }

    // TODO: Move this to a separate function
    player.current_exp += reward_exp as i32;
    player.maximum_exp = f32::floor(50f32 * f32::powi(1.2f32, player.level - 1)) as i32;

    while player.current_exp >= player.maximum_exp {
        player.level += 1;
        player.current_exp -= player.maximum_exp;

        if player.level % 5 == 0 {
            reward_diamonds += f32::floor((player.level as f32) * 0.8f32) as i32;
        }

        if player.level >= 30 {
            player.level = 30;
            player.current_exp = 0;
            break;
        }
    }

    player.maximum_exp = f32::floor(50f32 * f32::powi(1.2f32, player.level - 1)) as i32;
    player.diamond += reward_diamonds;
    let player_update = player::ActiveModel {
        id: Unchanged(player.id),
        level: Set(player.level),
        current_exp: Set(player.current_exp),
        maximum_exp: Set(player.maximum_exp),
        diamond: Set(player.diamond),
        ..Default::default()
    };

    score::ActiveModel {
        beatmap_id: Set(song_data.song_id),
        player_id: Set(session.player_id.unwrap()),
        mode: Set(mode),
        difficulty: Set(diff),
        finish_level: Set(play_data.finish_level),
        score: Set(play_data.score),
        is_full_combo: if play_data.is_full_combo == 1 {
            Set(true)
        } else {
            Set(false)
        },
        is_perfect: if play_data.is_all_max == 1 {
            Set(true)
        } else {
            Set(false)
        },
        miss_count: Set(play_data.miss),
        submitted_at: Set(now),
        ..Default::default()
    }
    .save(&db)
    .await?;

    prelude::Player::update(player_update).exec(&db).await?;
    prelude::PlayerCharacter::update(player_character::ActiveModel {
        player_id: Unchanged(player.id),
        character_id: Unchanged(player.selected_character_id),
        level: Set(character.level),
        experience: Set(character.experience),
        play_count: Set(character.play_count),
    })
    .exec(&db)
    .await?;

    let settle_data = SettleData {
        change_list: vec![],
        update_list: vec![SettleItemData {
            r#type: 2,
            count: player.diamond,
            id: 420,
        }],
        exp_data: Some(PlayerExpData {
            level: player.level,
            cur_exp: player.current_exp,
            max_exp: player.maximum_exp,
        }),
    };

    let single_song_data = RetFinishSong {
        song_info: proto::comet_scene::SingleSongInfo {
            song_id: song_data.song_id,
            finish_level: play_data.finish_level,
            score: play_data.score,
            is_full_combo: play_data.is_full_combo,
            is_all_max: play_data.is_all_max,
            miss: play_data.miss,
            play_count: (scores.len() as i32) + 1,
        },
        settle_data: Some(settle_data),
        new_rank: 0,
    };

    let full_data = get_player_full_data(player.id, &db).await?;

    Ok(vec![
        Response {
            main_cmd: MainCmd::Game,
            para_cmd: ParaCmd::CometScene(CometScene::ResponseFinishSong),
            body: single_song_data.encode_to_vec(),
        },
        Response {
            main_cmd: MainCmd::Game,
            para_cmd: ParaCmd::CometScene(CometScene::NotifyCharacterFullData),
            body: full_data.encode_to_vec(),
        },
    ])
}
