use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct BotInfo {
    pub token: String,
    pub prefix: String,
    pub ignore_bots: bool,
    pub owners_ids: Vec<u64>,
}