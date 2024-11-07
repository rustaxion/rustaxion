use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BPMRange {
    pub min: u32,
    pub max: u32,
}

impl Default for BPMRange {
    fn default() -> Self {
        Self { min: 0, max: 0 }
    }
}
