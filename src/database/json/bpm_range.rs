use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BPMRange {
    pub min: u32,
    pub max: u32,
}
