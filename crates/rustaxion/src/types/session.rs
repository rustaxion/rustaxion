#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NowPlaying {
    pub song_id: i32,
    pub mode: i32,
    pub difficulty: i32,
    pub time: i64,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct SessionData {
    pub account_id: Option<i32>,
    pub player_id: Option<i32>,
    pub now_playing: Option<NowPlaying>,
}

impl SessionData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
