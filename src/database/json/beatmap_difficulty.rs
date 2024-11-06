use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Difficulty {
    pub diff: u16,
    pub notes: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeatmapDifficulty {
    pub easy: Difficulty,
    pub normal: Difficulty,
    pub hard: Difficulty,
}
