use std::sync::Arc;

use anyhow::Context;
use prost::Message;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use tokio::sync::Mutex;

use crate::{
    database::entities::{
        player, prelude::*,
        score,
        sea_orm_active_enums::{BeatmapDifficulty, BeatmapMode},
    },
    types::{response::Response, session::SessionData},
};

use proto::comet_scene::{ReqSingleSongRank, RetSingleSongRank, SingleSongRankData};
use proto::enums::comet::{comet_scene::CometScene, MainCmd, ParaCmd};

#[rustfmt::skip]
pub async fn handle(_session: Arc<Mutex<SessionData>>, db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqSingleSongRank::decode(body.as_slice()).context("Failed to decode ReqSingleSongRank.")?;

    let mode = match req.mode() {
        proto::comet_scene::BeatmapMode::Bmm4k => BeatmapMode::FourKeys,
        proto::comet_scene::BeatmapMode::Bmm6k => BeatmapMode::SixKeys,
        proto::comet_scene::BeatmapMode::Bmm8k => BeatmapMode::EightKeys,
    };

    let difficulty = match req.difficulty() {
        proto::comet_scene::BeatmapDifficulty::BmdEasy => BeatmapDifficulty::Easy,
        proto::comet_scene::BeatmapDifficulty::BmdNormal => BeatmapDifficulty::Normal,
        proto::comet_scene::BeatmapDifficulty::BmdHard => BeatmapDifficulty::Hard,
    };

    let scores = Score::find()
        .filter(score::Column::BeatmapId.eq(req.song_id))
        .filter(score::Column::Mode.eq(mode))
        .filter(score::Column::Difficulty.eq(difficulty))
        .order_by_desc(score::Column::Score)
        .all(&db)
        .await?;

    // Keep only the best score per player
    let mut seen_players: std::collections::HashSet<i32> = std::collections::HashSet::new();
    let mut best_scores = Vec::new();
    for s in scores {
        if seen_players.insert(s.player_id) {
            best_scores.push(s);
        }
    }

    let mut rank_list = Vec::new();
    for (i, s) in best_scores.iter().take(100).enumerate() {
        let player = Player::find_by_id(s.player_id).one(&db).await?;
        if let Some(p) = player {
            rank_list.push(SingleSongRankData {
                rank: (i as i32) + 1,
                char_name: p.name,
                score: s.score,
                finish_level: s.finish_level,
                head_id: p.head_id,
                char_id: p.id as u64,
                country: p.country.into_proto() as i32,
                team_name: None,
                title_id: Some(p.title_id),
            });
        }
    }

    let ret = RetSingleSongRank { list: rank_list };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseSingleSongRank),
        body: ret.encode_to_vec(),
    }])
}
