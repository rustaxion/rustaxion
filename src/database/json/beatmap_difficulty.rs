use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Difficulty {
    pub diff: u16,
    pub notes: u32,
}

impl Default for Difficulty {
    fn default() -> Self {
        Self { diff: 1, notes: 1 }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeatmapDifficulty {
    pub easy: Difficulty,
    pub normal: Difficulty,
    pub hard: Difficulty,
}

impl BeatmapDifficulty {
    pub fn new(
        easy_diff: u16,
        easy_notes: u32,
        normal_diff: u16,
        normal_notes: u32,
        hard_diff: u16,
        hard_notes: u32,
    ) -> Self {
        Self {
            easy: Difficulty {
                diff: easy_diff,
                notes: easy_notes,
            },
            normal: Difficulty {
                diff: normal_diff,
                notes: normal_notes,
            },
            hard: Difficulty {
                diff: hard_diff,
                notes: hard_notes,
            },
        }
    }
}

impl Default for BeatmapDifficulty {
    fn default() -> Self {
        Self {
            easy: Difficulty::default(),
            normal: Difficulty::default(),
            hard: Difficulty::default(),
        }
    }
}
