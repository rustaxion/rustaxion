#[derive(Debug, Default)]
pub struct SessionData {
    pub account_id: Option<i32>,
}

impl SessionData {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
}
