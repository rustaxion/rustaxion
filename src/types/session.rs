#[derive(Debug, Default)]
pub struct SessionData {
    pub account_id: Option<i64>,
    pub player_id: Option<i64>,
}

impl SessionData {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
}
